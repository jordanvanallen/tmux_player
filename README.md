# tmux_player
Shows the current playing Song, Artist (or both) of your player in your Tmux status line (Intended for spotify use on Linux)

## Usage

1. Pull the repository `git pull https://github.com/jordanvanallen/tmux_player`
1. cd into the project `cd tmux_player`
1. Build a release binary `cargo build --release`
1. Copy the release binary to a directory in your $PATH `cp ./target/release/tmux_spotify somewhere/in/your/path`
1. Add the function to your Tmux status line with something like `set -g status-right "#(tmux_player both) | %Y-%m-%d %H:%M"` (You can also use either `#(tmux_player artist)` or `#(tmux_player song)` for just the artist or song respectively)
