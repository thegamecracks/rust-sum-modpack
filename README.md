# rust-sum-modpack

![](https://img.shields.io/github/actions/workflow/status/thegamecracks/rust-sum-modpack/clippy.yml?style=flat-square&label=clippy)
![](https://img.shields.io/github/actions/workflow/status/thegamecracks/rust-sum-modpack/test.yml?style=flat-square&label=tests)

A rewrite of my [sum_modpack.py] script in Rust, for fun!

```rust
$ cargo run -q -- --help
Fetch filesize statistics for a set of Steam Workshop IDs.

Usage: sum-modpack.exe [OPTIONS] <MODPACK>

Arguments:
  <MODPACK>  The modpack file to read

Options:
  -s, --sort <SORT>  How mods are sorted in the table [possible values: largest, smallest, title]
  -v, --verbose...   Increase logging verbosity
  -q, --quiet...     Decrease logging verbosity
  -h, --help         Print help
  -V, --version      Print version

$ cargo run -q -- examples/WHF_Drakovac.html
 INFO  sum_modpack > Fetching 19 workshop mods
╭────┬────────────┬──────────────┬─────────┬───────────────────────────────────╮
│  # │ Total (up) │ Total (down) │    Size │ Title                             │
├────┼────────────┼──────────────┼─────────┼───────────────────────────────────┤
│  1 │    4,952MB │      3,934MB │ 3,934MB │ JSRS SOUNDMOD 2025                │
│  2 │    1,018MB │      4,296MB │   362MB │ Drakovac                          │
│  3 │      656MB │      4,640MB │   345MB │ SSD Death Screams 2.1             │
│  4 │      312MB │      4,741MB │   102MB │ Project SFX: Remastered           │
│  5 │      211MB │      4,830MB │    90MB │ New CSAT (Overhaul) DVK           │
│  6 │      122MB │      4,893MB │    63MB │ Death and Hit reactions           │
│  7 │       59MB │      4,925MB │    32MB │ Project SFX: Footsteps            │
│  8 │       27MB │      4,941MB │    17MB │ Advanced Vault System: Remastered │
│  9 │       11MB │      4,946MB │     5MB │ CBA_A3                            │
│ 10 │        7MB │      4,948MB │     3MB │ Hide Among The Grass - HATG       │
│ 11 │        4MB │      4,950MB │     2MB │ Alternative Running               │
│ 12 │        2MB │      4,951MB │     2MB │ Pylon Manager                     │
│ 13 │      659KB │      4,951MB │   287KB │ Vehicle Inventory System          │
│ 14 │      372KB │      4,951MB │   130KB │ WBK Simple Blood                  │
│ 15 │      243KB │      4,952MB │   107KB │ Better Inventory                  │
│ 16 │      137KB │      4,952MB │    64KB │ CH View Distance                  │
│ 17 │       73KB │      4,952MB │    56KB │ Drongos Grenade Tweaks            │
│ 18 │       17KB │      4,952MB │    15KB │ Automatic Warning Suppressor      │
│ 19 │        3KB │      4,952MB │     3KB │ Splendid Smoke                    │
╰────┴────────────┴──────────────┴─────────┴───────────────────────────────────╯
```

The output of `sum_modpack.py` for comparison:

```py
$ python sum_modpack.py --help
usage: sum_modpack.py [-h] [-s {largest,smallest,title}] [-v] mods [mods ...]

Produce filesize statistics from a list of workshop mods.

positional arguments:
  mods                  Workshop item IDs, collections, or modpack files to sum

options:
  -h, --help            show this help message and exit
  -s, --sort {largest,smallest,title}
                        How mods are sorted in the table
  -v, --verbose         Increase logging verbosity

$ python sum_modpack.py examples/WHF_Drakovac.html
Number of mods: 19
  #  Total (up)  Total (down)       Size  Title
  1     4,952MB       3,934MB    3,934MB  JSRS SOUNDMOD 2025
  2     1,018MB       4,296MB      362MB  Drakovac
  3       656MB       4,640MB      345MB  SSD Death Screams 2.1
  4       312MB       4,741MB      102MB  Project SFX: Remastered
  5       211MB       4,830MB       90MB  New CSAT (Overhaul) DVK
  6       122MB       4,893MB       63MB  Death and Hit reactions
  7        59MB       4,925MB       32MB  Project SFX: Footsteps
  8        27MB       4,941MB       17MB  Advanced Vault System: Remastered
  9        11MB       4,946MB        5MB  CBA_A3
 10         7MB       4,948MB        3MB  Hide Among The Grass - HATG
 11         4MB       4,950MB        2MB  Alternative Running
 12         2MB       4,951MB        2MB  Pylon Manager
 13       659KB       4,951MB      287KB  Vehicle Inventory System
 14       372KB       4,951MB      130KB  WBK Simple Blood
 15       243KB       4,952MB      107KB  Better Inventory
 16       137KB       4,952MB       64KB  CH View Distance
 17        73KB       4,952MB       56KB  Drongos Grenade Tweaks
 18        17KB       4,952MB       15KB  Automatic Warning Suppressor
 19         3KB       4,952MB        3KB  Splendid Smoke
```

## License

This project is written under the [MIT License].

[sum_modpack.py]: /sum_modpack.py
[MIT License]: /LICENSE
