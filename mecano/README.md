# Mecano

A minimalist typing test.

![Live Preview](https://github.com/alvarojsino813/mecano/blob/main/assets/mecano.gif)

## Installation

You can install it with any AUR package manager, like yay:

### AUR

```bash
yay -Sy mecano
```

### Cargo

```bash
cargo install mecano
```

This way the executable will be installed at `~/.cargo/bin/mecano`.

You can add that path to your paths. Example for linux:

In `.bashrc`:
```bash
export PATH="$HOME/.cargo/bin:$PATH"
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

> Tip: You can change the `TextBox` size using the arrow keys.

## Configuration

You can change some default values like the theme, the time or some starting values at `~/.config/mecano/config.toml`.

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

## Coming soon

- [ ] A Wikipedia mode. It takes a random article from wikipedia and you complete it.
- [ ] A command mode. The std of a command will be taken by mecano and given to you as text to complete.
- [ ] More customization. Border colors, hiding or showing them...
- [ ] Even more customization. Full control over the layout in-game.
- [ ] Some fanciness. A title screen.
- [ ] Stats. More descriptive stats about your test.
- [ ] Better CLI. Autocomplete.
