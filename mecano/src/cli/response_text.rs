pub fn help_text() -> &'static str {
"Mecano, a typing train

Usage: mecano [OPTIONS] [FLAGS]

OPTIONS:
    -f, --file <FILE>           Plays in file mode with the chosen file
    -d, --dictionary <FILE>     Plays in dictionary mode with the chosen file
    -t, --time <SECS>           Choose the game time in seconds

FLAGS:
    -v, --version               Print version 
    -h, --help                  Print help
    -l --list-dictionaries      List all dicitonaries. You can add more at ~/.config/mecano/dictionaries"
}
