use std::{env, process};
use spotify_downloader::{config::config::Config, spotify::{playlist::playlist::Playlist, creds::creds::Creds}};

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(1)
    });

    let credentials = Creds::new(&config)
        .unwrap_or_else(|err| {
            println!("{}", err);
            process::exit(1)
        });
    let spotify = credentials
        .login()
        .await
        .unwrap_or_else(|err| {
            println!("{}", err);
            process::exit(1)
        });

    let playlist = Playlist::new(spotify, &config.playlist_id)
        .await
        .unwrap_or_else(|err| {
            println!("{}", err);
            process::exit(1)
        });
    println!("Found {:#?} tracks", playlist.tracks.len());

    playlist.download_tracks(&config.path).await;
    println!("Downloaded tracks");
}