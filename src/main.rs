use failure::{Error, ResultExt};
use structopt::StructOpt;
use mpris::PlayerFinder;

use std::str::FromStr;

#[derive(Debug)]
enum UserChoice {
    Metadata(Metadata),
    PlayerInteraction(PlayerInteraction)
}

#[derive(Debug)]
enum Metadata {
    Artist,
    Song,
    Both
}

#[derive(Debug)]
enum PlayerInteraction {
    PausePlay,
    Stop,
    Next,
    Previous
}

impl FromStr for UserChoice {
    type Err = String;

    fn from_str(val: &str) -> Result<Self, Self::Err> {
        match val {
            // --- Metadata ---
            "artist" => Ok(UserChoice::Metadata(Metadata::Artist)),
            "song" => Ok(UserChoice::Metadata(Metadata::Song)),
            "both" => Ok(UserChoice::Metadata(Metadata::Both)),

            // --- Player interations ---
            "pause_play" => Ok(UserChoice::PlayerInteraction(PlayerInteraction::PausePlay)),
            "stop" => Ok(UserChoice::PlayerInteraction(PlayerInteraction::Stop)),
            // Change song
            "next" => Ok(UserChoice::PlayerInteraction(PlayerInteraction::Next)),
            "previous" => Ok(UserChoice::PlayerInteraction(PlayerInteraction::Previous)),

            _ => Err("Can't parse input".to_string()),
        }
    }
}

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(help = "Select what you want returned, 'artist', 'song', 'both', or 'pause'/'play' if you wish to interact with the player")]
    user_choice: UserChoice,
}

fn main() {
    let user_choice = Opt::from_args().user_choice;

    match user_choice {
        UserChoice::Metadata(_) => match print_player_metadata(user_choice) {
            Ok(_) => (),
            Err(_error) => std::process::exit(1),
        },
        UserChoice::PlayerInteraction(_) => match interact_with_player(user_choice) {
            Ok(_) => (),
            Err(_error) => std::process::exit(2),
        },
    };
}

fn interact_with_player(user_choice: UserChoice) -> Result<(), Error> {
    let player = get_player()?;

    match user_choice {
        UserChoice::PlayerInteraction(PlayerInteraction::PausePlay) => Ok(player.play_pause().context("Could not pause/play the player")?),
        UserChoice::PlayerInteraction(PlayerInteraction::Stop) => Ok(player.stop().context("Could not stop the player")?),
        UserChoice::PlayerInteraction(PlayerInteraction::Next) => Ok(player.next().context("Could not get next song")?),
        UserChoice::PlayerInteraction(PlayerInteraction::Previous) => Ok(player.previous().context("Could not get previous song")?),
        _ => Ok(()),
    }
}

fn print_player_metadata(user_choice: UserChoice) -> Result<(), Error> {
    let player = get_player()?;

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

    let status = player
        .get_playback_status()
        .context("Could not get playback status")?;

    let playback_symbol = match status {
        mpris::PlaybackStatus::Playing => "▶",
        mpris::PlaybackStatus::Paused => "❙❙",
        mpris::PlaybackStatus::Stopped => "◼"
    };

    match user_choice {
        UserChoice::Metadata(Metadata::Artist) => Ok(println!("{} {}", playback_symbol, artists[0])),
        UserChoice::Metadata(Metadata::Song) => Ok(println!("{} {}", playback_symbol, song_name)),
        UserChoice::Metadata(Metadata::Both) => Ok(println!("{} {} - {}", playback_symbol, artists[0], song_name)),
        _ => Ok(())
    }
}

fn get_player() -> Result<mpris::Player<'static>, Error> {
    let player_finder = PlayerFinder::new()
        .context("Could not connect to D-Bus")?;

    Ok(player_finder
        .find_active()
        .context("Could not find an active player")?)
}
