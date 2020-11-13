use failure::{Error, ResultExt};
use structopt::StructOpt;
use mpris::PlayerFinder;

use std::str::FromStr;

#[derive(Debug)]
enum UserChoice {
    Metadata(Metadata),
    PlayerInteraction(PlayerInteraction),
    Volume(Volume),
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

#[derive(Debug)]
enum Volume {
    Get,
    Raise,
    Lower,
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

            // --- Volume ---
            "get_volume" => Ok(UserChoice::Volume(Volume::Get)),
            "raise_volume" => Ok(UserChoice::Volume(Volume::Raise)),
            "lower_volume" => Ok(UserChoice::Volume(Volume::Lower)),

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
        UserChoice::Volume(_) => match player_volume(user_choice) {
            Ok(_) => (),
            Err(_error) => std::process::exit(3),
        }
    };
}

fn player_volume(user_choice: UserChoice) -> Result<(), Error> {
    let player = get_player()?;

    let current_volume = player.get_volume().context("Could not get player volume")?;

    match user_choice {
        UserChoice::Volume(Volume::Get) => Ok(println!("{:.1}", current_volume * 100.0)),
        UserChoice::Volume(Volume::Raise) => {
            let new_volume = current_volume + 0.05;
            println!("New volume is: {}", new_volume);
            player.set_volume(new_volume.max(1.0)).context("Could not raise volume")?;
            Ok(())
        },
        UserChoice::Volume(Volume::Lower) => {
            let new_volume = current_volume - 0.05;
            player.set_volume(new_volume.min(0.0)).context("Could not lower volume")?;
            Ok(())
        },
        _ => Ok(()),
    }
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
