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
Alternatively, put the binary from the Releases page somewhere in your $PATH (or %PATH%)

# Configuration
Configuration is in a `Devloop.toml` (by default) file in the current directory.

It is a toml file. Here's an example (from this repository's `Devloop.toml`), along with explanations:

```toml
# This will be printed when you press q to quit
reminders = "Don't forget to format!"

# The `[[name]]` syntax in toml means that you're defining an array of tables.
# So this next section is equivalent to:
#
# tasks = [{ name = "Clippy", command = "cargo clippy -q" }]
#
# Each `[[section]]` will define one *element* in the array.

# Each task will be run in order every time you press Enter.
[[tasks]]
name = "Clippy"
command = "cargo clippy -q"

# Actions is a table where the key is the shortcut (in this case "f")
[actions.f]
name = "Format"
command = "cargo fmt"

[actions.t]
name = "Test"
command = "cargo test -q"

[actions.b]
name = "Benchmark"
command = "cargo bench -q"
pause = true  # devloop will wait for another Enter before re-running [[tasks]]

[actions.R]
name = "Build release"
command = "cargo build --release"

[actions.r]
name = "Run"
command = "cargo run"  # Another good candidate for pause=true

[actions.c]
name = "Clean"
command = "cargo clean"
```
