use rspotify::{model::*, prelude::*, ClientCredsSpotify, Credentials};
use std::{fs, env, collections::HashMap, path::PathBuf};
use ytd_rs::{YoutubeDL, Arg, YoutubeDLResult};

#[tokio::main]
async fn main() {
    let path = "./tracks";
    let playlist_id = playlist_id();

    let playlist = playlist(playlist_id).await;

    let tracks = playlist_tracks(&playlist);
    println!("Found {:#?} tracks\n", tracks.len());
    
    for track in tracks {
        let track_name = track.0.split(" - ").collect::<Vec<&str>>()[0];

        if check_dup(&path, &track_name) {
            println!("Skipped {}", track_name);
            continue;
        } else {
            println!("Downloading {} by {}", track_name, track.1);
        }

        download(&path, &track_name, &track.1);

        println!("Downloaded {} by {}", track_name, track.1);
    }
}

fn playlist_id() -> String {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Missing playlist link");
    }

    let start_bytes = args[1]
        .find("playlist/")
        .unwrap_or(0);
    let end_bytes = args[1]
        .find("?si=")
        .unwrap_or(args[1].len());

    args[1][start_bytes + 9..end_bytes]
        .to_string()
        .clone()
}

async fn playlist(playlist_id: String) -> FullPlaylist {
    let spotify = login().await;
    let playlist_uri = PlaylistId::from_uri(&format!("spotify:playlist:{playlist_id}")).unwrap();
    spotify
        .playlist(&playlist_uri, None, None)
        .await
        .unwrap()
}

fn playlist_tracks<'a>(playlist: &'a FullPlaylist) -> HashMap<&'a String, &'a String> {
    let mut tracks = HashMap::new();
    for item in playlist.tracks.items.iter() {
        let track = item.track
            .as_ref()
            .unwrap();
        if let PlayableItem::Track(full_track) = track {
            tracks.insert(&full_track.name, &full_track.artists[0].name);
        }
    }
    tracks
}

fn check_dup(path: &str, track_name: &str) -> bool {
    let paths = fs::read_dir(path).unwrap();
    for path in paths {
        let simple_name = track_name
            .clone()
            .to_lowercase()
            .chars()
            .filter(|s| !s.is_whitespace())
            .collect::<String>();
        let simple_filename = path
            .unwrap()
            .file_name()
            .into_string()
            .unwrap()
            .to_lowercase()
            .chars()
            .filter(|s| !s.is_whitespace())
            .collect::<String>();
        if simple_filename.contains(simple_name.as_str()) {
            return true
        }
    }
    false
}

fn download(path: &str, track_name: &str, track_artist: &str) -> YoutubeDLResult {
    let args = vec![
        Arg::new("--quiet"),
        Arg::new("--extract-audio"),
        Arg::new("--embed-thumbnail"),
        Arg::new("--add-metadata"),
        Arg::new_with_arg("--default-search", "ytsearch"),
        Arg::new_with_arg("--downloader", "aria2c"),
        Arg::new_with_arg("--audio-format", "mp3"),
        Arg::new_with_arg("--audio-quality", "0"),
        Arg::new_with_arg("--output", "%(artist)s - %(track)s.%(ext)s"),
    ];
    let path = PathBuf::from(path);
    let ytd = YoutubeDL::new(&path, args, format!("{} {}", track_name, track_artist).as_str())
        .unwrap();
    ytd
        .download()
        .unwrap()
}

async fn login() -> ClientCredsSpotify {
    let creds = Credentials {
        id: "7f5c956cbf674174aab45d529a8a5d39".to_string(),
        secret: Some("43054d2e116f4179bd7e5cc8ec2b335e".to_string())
    };
    let mut spotify = ClientCredsSpotify::new(creds);
    spotify
        .request_token()
        .await
        .unwrap();
    spotify
}