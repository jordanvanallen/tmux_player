use structopt::StructOpt;
use mpris::PlayerFinder;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(help = "Select what you want returned, 'artist', 'song', or 'both'")]
    song_attributes: String,
}

fn main() {
    let opt = Opt::from_args();

    match opt.song_attributes.as_str() {
        "artist" | "song" | "both" => (),
        _ => eprintln!("Valid parameters are 'artist', 'song', or 'both'"),
    }

    let player = PlayerFinder::new()
        .expect("could not connect to DBus")
        .find_active()
        .expect("could not find any player");

    let metadata  = player.get_metadata().expect("could not get player metadata");

    match metadata.album_artists() {
        Some(_) => print_spotify_metadata(opt, metadata),
        None => (),
    }
}

fn print_spotify_metadata(opt: Opt, metadata: mpris::Metadata) {
    let artists   = metadata.artists().expect("could not find artists");
    let song_name = metadata.title().expect("Could not find song name");

    match opt.song_attributes.as_str() {
        "artist" => println!("♫ {}", artists[0]),
        "song" => println!("♫ {}", song_name),
        "both" => println!("♫ {} - {}", artists[0], song_name),
        _ => (),
    }
}
