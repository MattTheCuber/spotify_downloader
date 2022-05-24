pub mod config {
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum ConfigError {
        #[error("You must either specify client id in your command arguments or in your environment variables.\nRefer to the README for more information.\n")]
        MissingClientId,
        #[error("You must either specify client secret in your command arguments or in your environment variables.\nRefer to the README for more information.\n")]
        MissingClientSecret,
    }

    pub struct Config {
        pub playlist_id: String,
        pub path: String,
        pub client_id: Option<String>,
        pub client_secret: Option<String>,
    }
    
    impl Config {
        pub fn new(args: &[String]) -> Result<Config, &'static str> {
            if args.len() < 2 {
                return Err("Missing link argument. Please use the format:\n spotify_downloader <playlist_link> <path> (client_id) (client_secret)");
            } else if args.len() < 3 {
                return Err("Missing path argument. Please use the format:\n spotify_downloader <playlist_link> <path> (client_id) (client_secret)");
            }
    
            let start_bytes = args[1]
                .find("playlist/")
                .unwrap_or(0);
            let end_bytes = args[1]
                .find("?si=")
                .unwrap_or(args[1].len());
            let playlist_id = args[1][start_bytes + 9..end_bytes]
                .to_string()
                .clone();
    
            let path = args[2].clone();
    
            let client_id = if args.len() >= 4 {
                Some(args[3].clone())
            } else {
                None
            };
            let client_secret = if args.len() >= 5 {
                Some(args[4].clone())
            } else {
                None
            };
    
            let config = Config {
                playlist_id,
                path,
                client_id,
                client_secret,
            };
            Ok(config)
        }
    }
}