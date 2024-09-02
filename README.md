# Mecano

A minimalistic typing test

## Installation

You can install it with any AUR package manager, like yay:

```bash
yay -Sy mecano
```

## Usage

Just execute and type!

```
Mecano, a typing train

Usage: mecano [OPTIONS] [FLAGS]

OPTIONS:
-f, --file <FILE>           Plays using the chosen file or dictionary
-m, --mode <MODE>           Plays the chosen mode
-r, --rate <RATE>           Plays with the chosen rate. This affects time measures accuracy. The higher the better.
-t, --time <SECS>           Choose the game time in seconds

FLAGS:
-h, --help                  Print help
-v, --version               Print version 
    --list-dictionaries     List all dicitonaries. You can add more at ~/.config/mecano/dictionaries
    --list-modes            List all available modes
```

## Configuration

You can change some default values like the theme, the time or some starting values at `~/.config/mecano/config.toml`.
The default configuration is in `/usr/share/mecano/config.toml`.

```toml
# Maximum width of each line
width = 80

# Game time for the test
max_time = 60

# Max lines shown
lenght = 2

# File from which words are taken
file = "100_english"

# Play mode. [ dictionary | file ]
mode = "dictionary"

# update rate: 1000 suggested value
rate = 1000

# Color theme (more coming soon)
[theme] # Uncomment this line to change theme

# Selected char color
selected = "#888888"

# Wrong char color
wrong = "#FF8888"

# Right char color
right = "#44FF44"
```

- Modify the configuration file in `~/.config/mecano/mecano.toml`

- Add more dictionaries at `~/.config/mecano/dictionaries/`

- Use any file on your system

## Motivation

I wanted to undergo the full process of developing, publishing and maintining a software project.

I was thinking about something minimalistic, and I used to do typing test on the web, so I thought making one for the terminal could be a nice idea.

Part of the challenge was to develop `mecano` with almost no use of other crates.

Don't hesitate on suggesting any recommendation, issue or PR.
