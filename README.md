# mtg-proxy-boosterpack-gen

This project exists for easy generation of **Magic: The Gathering** booster packs
proxies for casual play or draft sessions.

It uses [Reuben's algorhitm](https://gist.github.com/fenhl/8d163733ab92ed718d89975127aac152#reubens-algorithm) to generate packs that mimic the common patterns and rules of official MTG boosters.

## Usage

To go through available sets take a look at [api.scryfall.com/sets](https://api.scryfall.com/sets). Take the `code` value to pass it to the command below.

> Note: not all sets can generate a valid booster pack (10x common, 3x uncommon, 1x mythic/rare, 1x land). At this point this edge case isn't supported. Some verified ones include Bloomburrow (blb), Teenage Mutant Ninja Turtles (tmt), Avatar: The Last Airbender (tla).

Build and run

```sh
cargo run --release <set code> <booster pack amount>
```

Example run:

```sh
$ cargo run --release blb 3
Loaded cards from cache
Loaded sets from cache
Using set: Bloomburrow (blb)
Building card pools...
Generating 3 booster packs...
(...)
```

This will generate a `boosters.pdf` file which can be used to print proxies

## Acknowledgement

- Scryfall - for providing a fantastic API and card database.
- [fverdoja/booster-tutor](https://github.com/fverdoja/booster-tutor) - inspiration for print-run generation.
- [fenhl/lore-seeker-pack-spec.md](https://gist.github.com/fenhl/8d163733ab92ed718d89975127aac152#reubens-algorithm) - Reuben's algorhitm.


> Wizards of the Coast, Magic: The Gathering, and their logos are trademarks of
Wizards of the Coast LLC in the United States and other countries. © 1993-2026
Wizards. All Rights Reserved. mtg-proxy-boosterpack-gen may use the trademarks
and other intellectual property of Wizards of the Coast LLC, which is permitted
under Wizards' Fan Site Policy. For more information about Wizards of the Coast
or any of Wizards' trademarks or other intellectual property, please visit
their [website](https://company.wizards.com/en).
