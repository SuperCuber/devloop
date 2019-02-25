# Description

Ever found yourself pressing the up arrow and enter 10 times a minute?
Ever found yourself typing the same commands again and again?

No worries! Devloop to the rescue!

# Usage
I've created this tool to allow a very simple workflow:
Open a terminal/editor for editing code, another terminal for devloop.

Devloop allows you to recompile (and test, and whatever you want!) with just pressing Enter,
and also save a lot of shortcuts to do other useful and repetitive commands.

Originally it was a [shell script](https://github.com/SuperCuber/dotfiles/blob/62437b10abbfc509421b4ff4b4122547d8b263e8/scripts/devloop) but the configuration file being a shell script bothered me, so I made it a rust program!

# Installation
Devloop is on crates.io, so just `cargo install devloop` after installing Rust.

# Configuration
Configuration is in a `dev_loop` (by default) file in the current directory.

It is a yaml file. For an example, check out the `dev_loop` file in this repository (which I used to develop this utility by the way).
