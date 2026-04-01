use printpdf::*;
use std::{thread, time::Duration};
use reqwest::blocking::Client;
use crate::model::Card;
use crate::bulk::API_DELAY_MS;

const PAGE_W_MM:      f32 = 210.0;
const PAGE_H_MM:      f32 = 297.0;
const CARD_W_MM:      f32 = 63.0;
const CARD_H_MM:      f32 = 88.0;
const CARD_GAP_MM:    f32 = 3.0;
const PAGE_MARGIN_MM: f32 = 7.5;  // Left/top margin (I didnt care enough to center)

pub fn generate_pdf(cards: &[Card], filename: &str) {
    let mut doc = PdfDocument::new("MTG Proxies");
    let mut pages_ops: Vec<Vec<Op>> = Vec::new();

    // Build a single client once, outside the loop
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .unwrap();

    for (i, card) in cards.iter().enumerate() {
        println!("Processing card {}/{}", i + 1, cards.len());

        let page_index = i / 9; // 9 cards per page
        let card_in_page = i % 9;
        let col = card_in_page % 3;
        let row = card_in_page / 3;

        match card.image_url("normal") {
            Some(url) => {
                println!("  Fetching image from URL: {}", url);

                let mut attempts = 0;
                let img_bytes = loop {
                    match client.get(url).send() {
                        Ok(resp) => match resp.bytes() {
                            Ok(b) => break b,
                            Err(e) => {
                                eprintln!("    Failed to read bytes: {:?}", e);
                            }
                        },
                        Err(e) => {
                            eprintln!("    Failed to fetch URL: {:?}", e);
                        }
                    }

                    attempts += 1;
                    if attempts >= 3 {
                        eprintln!("    Failed after 3 attempts, skipping this card");
                        break Vec::new().into(); // skip
                    }

                    thread::sleep(Duration::from_millis(500)); // retry delay
                    println!("    Retrying...");
                };

                // --- ADD API_DELAY_MS SLEEP HERE ---
                thread::sleep(Duration::from_millis(API_DELAY_MS));

                if img_bytes.is_empty() {
                    continue; // skip this card
                }

                println!("  Decoding image ({} bytes)", img_bytes.len());
                let mut warnings = Vec::new();
                let raw_image = match printpdf::image_types::RawImage::decode_from_bytes(&img_bytes, &mut warnings) {
                    Ok(img) => {
                        img
                    },
                    Err(e) => {
                        eprintln!("    Failed to decode image: {:?}", e);
                        for w in &warnings {
                            eprintln!("    Warning: {:?}", w);
                        }
                        continue;
                    }
                };

                let image_id = doc.add_image(&raw_image);

                let pt_per_mm = 72.0 / 25.4;
                let assumed_dpi = 300.0;

                let pos_x_mm =
                    PAGE_MARGIN_MM + col as f32 * (CARD_W_MM + CARD_GAP_MM);

                let pos_y_mm =
                    PAGE_H_MM
                    - PAGE_MARGIN_MM
                    - CARD_H_MM
                    - row as f32 * (CARD_H_MM + CARD_GAP_MM);

                let pos_x_pt = pos_x_mm * pt_per_mm;
                let pos_y_pt = pos_y_mm * pt_per_mm;

                let target_w_pt = CARD_W_MM * pt_per_mm;
                let target_h_pt = CARD_H_MM * pt_per_mm;

                let image_w_pt = raw_image.width as f32 * 72.0 / assumed_dpi;
                let image_h_pt = raw_image.height as f32 * 72.0 / assumed_dpi;

                let scale_x = target_w_pt / image_w_pt;
                let scale_y = target_h_pt / image_h_pt;

                let transform = XObjectTransform {
                    translate_x: Some(Pt(pos_x_pt)),
                    translate_y: Some(Pt(pos_y_pt)),
                    scale_x: Some(scale_x),
                    scale_y: Some(scale_y),
                    ..Default::default()
                };

                if pages_ops.len() <= page_index {
                    pages_ops.push(Vec::new());
                }
                pages_ops[page_index].push(Op::UseXobject { id: image_id, transform });

                println!("  Added image to page {}", page_index + 1);
            }
            None => {
                println!("  No URL for this card, skipping");
            }
        }
    }

    if pages_ops.is_empty() {
        eprintln!("No images fetched, PDF will be empty!");
        return;
    }

    println!("Creating PDF pages...");
    let mut pdf_pages = Vec::new();
    for (i, ops) in pages_ops.into_iter().enumerate() {
        println!("  Page {} has {} images", i + 1, ops.len());
        let page = PdfPage::new(Mm(PAGE_W_MM), Mm(PAGE_H_MM), ops);
        pdf_pages.push(page);
    }

    let mut warnings = Vec::new();
    let pdf_bytes: Vec<u8> = doc.with_pages(pdf_pages).save(&PdfSaveOptions::default(), &mut warnings);
    std::fs::write(filename, pdf_bytes).unwrap();

    println!("PDF saved to {}", filename);
    if !warnings.is_empty() {
        println!("Warnings:");
        for w in warnings {
            println!("  {:?}", w);
        }
    }
}
