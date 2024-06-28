# Mecano

A minimalistic typing test

## Installation

Still working on uploading mecano to AUR. Meanwhile you can clone the repository and:

```bash
cargo run
```

## Usage

Just execute and type!

```bash
Mecano, a typing train

Usage: mecano [OPTIONS] [FLAGS]

OPTIONS:
    -f, --file <FILE>           Plays in file mode with the chosen file
    -d, --dictionary <FILE>     Plays in dictionary mode with the chosen file
    -t, --time <SECS>           Choose the game time in seconds

FLAGS:
    -v, --version               Print version 
    -h, --help                  Print help
    -l --list-dictionaries      List all dicitonaries. You can add more at ~/.config/mecano/dictionaries
```

- Modify the configuration file in `~/.config/mecano/mecano.toml`

- Add more dictionaries at `~/.config/mecano/dictionaries/`

- Use any file on your system

## Motivation

I wanted to undergo the full process of developing, publishing and maintining a software project.

I was thinking about something minimalistic, and I used to do typing test on the web, so I thought making one for the terminal could be a nice idea.

Part of the challenge was to develop `mecano` with almost no use of other crates.

Don't hesitate on suggesting any recommendation, issue or PR.
