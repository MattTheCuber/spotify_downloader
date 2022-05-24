pub mod creds {
    use std::{env};
    use rspotify::{ClientCredsSpotify, Credentials};
    use crate::config::config::{Config, ConfigError};

    pub struct Creds {
        pub client_id: String,
        pub client_secret: String,
    }

    impl Creds {
        pub fn new(config: &Config) -> Result<Creds, ConfigError> {
            let client_id = config.client_id
                .clone()
                .unwrap_or_else(|| env::var("RSPOTIFY_CLIENT_ID")
                    .map_err(|_err| ConfigError::MissingClientId)
                    .unwrap()
                );
                
            let client_secret = config.client_secret
                .clone()
                .unwrap_or_else(|| env::var("RSPOTIFY_CLIENT_SECRET")
                    .map_err(|_err| ConfigError::MissingClientSecret)
                    .unwrap()
                );
            
            Ok(Creds { client_id, client_secret })
        }

        pub async fn login(&self) -> Result<ClientCredsSpotify, &'static str> {
            let creds = Credentials {
                id: self.client_id.clone(),
                secret: Some(self.client_secret.clone())
            };
            let mut spotify = ClientCredsSpotify::new(creds);

            match spotify.request_token().await {
                Ok(_) => Ok(spotify),
                Err(_err) => Err("Could not login to Spotify. Please check your credentials."),
            }
        }
    }
}