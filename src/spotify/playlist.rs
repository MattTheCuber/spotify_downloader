pub mod playlist {
    use std::{collections::HashMap, path::PathBuf, fs};
    use rspotify::{model::*, prelude::*, ClientCredsSpotify};
    use ytd_rs::{Arg, YoutubeDL};

    pub struct Playlist {
        pub playlist: FullPlaylist,
        pub tracks: HashMap<String, String>,
    }
    
    impl Playlist {
        pub async fn new(spotify: ClientCredsSpotify, playlist_id: &str) -> Result<Playlist, &'static str> {
            let playlist_uri = PlaylistId::from_uri(&format!("spotify:playlist:{playlist_id}")).unwrap();
            let playlist_result = spotify
                .playlist(&playlist_uri, None, None)
                .await;
    
            let playlist = match playlist_result {
                Ok(playlist) => playlist,
                Err(_err) => return Err("Could not get playlist from Spotify. Please check your playlist link."),
            };
            let tracks = playlist_track_names(&playlist);
    
            Ok(Playlist {
                playlist,
                tracks,
            })
        }
    
        pub async fn download_tracks(&self, path: &str) {
            let tracks = self.filter_tracks(path);
            println!("Downloading {} tracks", tracks.len());

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
            let ytd = YoutubeDL::new_multiple_links(&path, args, tracks).unwrap();

            ytd.download().unwrap();
        }

        fn filter_tracks(&self, path: &str) -> Vec<String> {
            let mut filtered_tracks = vec![];
    
            for track in &self.tracks {
                let track_name = track.0.split(" - ").collect::<Vec<&str>>()[0];
        
                if check_dup(&path, &track_name) {
                    println!("Skipped {}", track_name);
                    continue;
                } else {
                    filtered_tracks.push(format!("{} {}", track_name, track.1));
                }
            }
            filtered_tracks
        }
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
    
    fn playlist_track_names(playlist: &FullPlaylist) -> HashMap<String, String> {
        let mut tracks = HashMap::new();
        for item in playlist.tracks.items.iter() {
            let track = item.track
                .clone()
                .unwrap();
            if let PlayableItem::Track(full_track) = track {
                tracks.insert(full_track.name, full_track.artists[0].name.clone());
            }
        }
        tracks
    }
}