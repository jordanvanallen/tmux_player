# tmux_player
Useful for those who live in TMUX (or those with window tiling mangers, polybar users, etc.) and want to see their current playing track, artist, or both.

In addition to this, can be used for interacting with the player to do things like skip to the next song, adjust the volume, or pause/play.

This script is intended for use with spotify, but seems to work fine with other audio players I've tested.

## Usage

### Build the binary

1. Pull the repository `git pull https://github.com/jordanvanallen/tmux_player`
1. cd into the project `cd tmux_player`
1. Build a release binary `cargo build --release`
1. Copy the release binary to a directory in your $PATH `cp ./target/release/tmux_spotify somewhere/in/your/path`

### Tmux status line
1. Add the function to your Tmux status line with something like `set -g status-right "#(tmux_player both) | %Y-%m-%d %H:%M"`
* (You can also use either `#(tmux_player artist)` or `#(tmux_player song)` for just the artist or song respectively)

### Interact with audio player
* Pause/Play your audio player using `tmux_player pause_play`
* Skip to the next song using `tmux_player next`
* Go back to the previous song by using `tmux_player previous`

#### Volume adjustment and fetching
* Get the current audio level of your player (as a percentage) using `tmux_player get_volume`
* Raise the volume by 5% using `tmux_player raise_volume`
* Lower the volume by 5% using `tmux_player lower_volume`
