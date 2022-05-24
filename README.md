# spotify_downloader

## Spotify application setup
- Visit the [Spotify Developer portal](https://developer.spotify.com/dashboard/applications)
- Create an app
- Copy the **Client ID** and **Client Secret** for later use

## Setup Environment variables
Replace `CLIENT_ID` and `CLIENT_SECRET` with your credentials from 
```
PS> $Env:RSPOTIFY_CLIENT_ID="CLIENT_ID";
PS> $Env:RSPOTIFY_CLIENT_SECRET="CLIENT_SECRET";
```

## Command Usage
Link and path are required, client id and client secret can be skipped if you setup environment variables
`spotify_downloader <playlist_link> <path> (client_id) (client_secret)`
