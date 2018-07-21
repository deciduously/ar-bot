# ar-bot

[![Build Status](https://travis-ci.org/deciduously/ar-bot.svg?branch=master)](https://travis-ci.org/deciduously/ar-bot)

Command-line tool for collection and batching of auto-generated emails, in an attempt to save paper.

Ive tested on Linux and Windows. It Probably works ok on MacOS, too, if you happen to be an alternate unverse me
with this exact need but not using either of those two operating systems.

## Usage

`ar-bot [FLAGS] [OPTIONS]`

FLAGS:

* `-h, --help`       Prints help information
* `-p, --preview`    Displays the current contents of the batch
* `-r, --report`     Daily report comparing inputs to outputs for the day
* `-V, --version`    Prints version information

OPTIONS:

* `-c, --config <CONFIG_FILE>`    Specify an alternate toml config file

Feel free to mix and match any of the above, it's fun.

With no config given it will default to `Bot.toml`, and with no flags or options passed it will print its configuration and quit.

## Dependencies

* Stable [rust](https://www.rust-lang.org)

## Crates

* [chrono](https://github.com/chronotope/chrono)
* [clap](https://github.com/kbknapp/clap-rs)
* [email-format](https://github.com/mikedilger/email-format)
* [error-chain](https://github.com/rust-lang-nursery/error-chain)
* [lazy_static](https://github.com/rust-lang-nursery/lazy-static.rs)
* [regex](https://github.com/rust-lang/regex)
* [serde/serde_derive](https://serde.rs)
* [toml](https://github.com/alexcrichton/toml-rs)
* [pretty_assertions](https://github.com/colin-kiegel/rust-pretty-assertions)
* [rand](https://github.com/rust-lang-nursery/rand)

## Notes

This is pretty gosh dang domain specific.
