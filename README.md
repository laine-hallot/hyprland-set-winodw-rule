## Dependencies

1. `slurp` - https://github.com/emersion/slurp/

## Install

1. Clone the repo
1. `cargo build --release`
1. Sym-link the executable at `./target/release/hyprland-window-rule` to some directory in your `$PATH` (e.g. `~/bin/)

## Usage

This is very unfinished so right now you can only generate a rule to make a window float based on its initial title.

1. `hyprland-window-rule generate --float`
1. Select a window with you mouse cursor
1. Copy the output into your hyprland config

## Know Issues

The mouse selection is a little finicky since it relies on slurp, which means it really easy to select a region instead of a window. Just go slow and make sure you don't drag your mouse at all while you click
