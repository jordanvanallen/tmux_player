use failure::{Error, ResultExt};
use structopt::StructOpt;
use mpris::PlayerFinder;

use std::str::FromStr;

#[derive(Debug)]
enum SongAttribute {
    Artist,
    Song,
    Both
}

impl FromStr for SongAttribute {
    type Err = String;

    fn from_str(val: &str) -> Result<Self, Self::Err> {
        match val {
            "artist" => Ok(SongAttribute::Artist),
            "song" => Ok(SongAttribute::Song),
            "both" => Ok(SongAttribute::Both),
            _ => Err("Can't parse input".to_string()),
        }
    }
}

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(help = "Select what you want returned, 'artist', 'song', or 'both'")]
    song_attribute: SongAttribute,
}

fn main() {
    match print_spotify_metadata() {
        Ok(_) => (),
        Err(_error) => {
            std::process::exit(1);
        }
    }
}

fn print_spotify_metadata() -> Result<(), Error> {
    let user_choice = Opt::from_args().song_attribute;

    let player_finder = PlayerFinder::new()
        .context("Could not connect to D-Bus")?;

    let player = player_finder
        .find_active()
        .context("Could not find an active player")?;

    let metadata  = player
        .get_metadata()
        .context("could not get player metadata")?;

    // Ensure the metadata has actual artist data
    match metadata.album_artists() {
        Some(_) => (),
        None => std::process::exit(1),
    }

    let artists   = metadata.artists().unwrap();
    let song_name = metadata.title().unwrap();

    match user_choice {
        SongAttribute::Artist => Ok(println!("♫ {}", artists[0])),
        SongAttribute::Song => Ok(println!("♫ {}", song_name)),
        SongAttribute::Both => Ok(println!("♫ {} - {}", artists[0], song_name))
    }
}
