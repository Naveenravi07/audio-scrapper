use audio_scrapper::*;
use clap::Parser;
use dotenvy::dotenv;
use rspotify::{prelude::OAuthClient, scopes, AuthCodeSpotify, Credentials, OAuth};
use std::env;
use std::{fs, process};

#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");

    let args = Args::parse();

    match args.method {
        InputMethod::File { path } => {
            let content_raw = fs::read_to_string(&path);
            let content = content_raw.unwrap_or_else(|err| {
                eprintln!("cannot read file");
                eprintln!("{}", err);
                process::exit(1);
            });
            let tracks = content.lines().map(|x|x.to_string()).collect::<Vec<_>>();
            downlaod_tracks_from_youtube(&tracks, &args.output_dir);
        }

        InputMethod::Spotify => {
            let client_id = env::var("client_id").unwrap_or_else(|err| {
                eprintln!("Cannot read client id from env file");
                eprintln!("{}", err);
                process::exit(1);
            });
            let client_secret = env::var("client_secret").unwrap_or_else(|err| {
                eprintln!("Cannot read client secret from env file");
                eprintln!("{}", err);
                process::exit(1);
            });

            let credentials = Credentials::new(&client_id, &client_secret);

            let oauth = OAuth {
                redirect_uri: "http://localhost:42069".to_string(),
                scopes: scopes!("user-read-private user-read-email"),
                ..Default::default()
            };

            let spotify = AuthCodeSpotify::new(credentials, oauth);
            let url = spotify.get_authorize_url(false).unwrap_or_else(|err| {
                eprintln!("\n Problem in autorizing spotify");
                eprintln!("{}", err);
                process::exit(1);
            });

            spotify.prompt_for_token(&url).await.unwrap_or_else(|err| {
                eprintln!("\n Invalid url");
                eprintln!("{}", err);
                process::exit(1);
            });

            println!("Fetching your playlists ");
            let playlist = spotify
                .current_user_playlists_manual(None, None)
                .await
                .unwrap_or_else(|err| {
                    eprintln!("\n Failed to fetch playlist");
                    eprintln!("{}", err);
                    process::exit(1);
                });

            for (index, track) in playlist.items.iter().enumerate() {
                println!("Press {} to download {}", index, track.name);
            }

            let mut inp_string = String::new();
            std::io::stdin()
                .read_line(&mut inp_string)
                .expect("Failed to read input");
            let num: usize = inp_string.trim().parse().unwrap();

            let playlisturl = playlist.items[num].id.to_string();

            println!("Downloading {}", playlist.items[num].name);

            let mut offset: u32 = 0;

            while {
                let results = fetch_tracks_of_playlist(&spotify, &playlisturl, Some(offset)).await;
                offset += 100;
                downlaod_tracks_from_youtube(&results.tracks, &args.output_dir);
                results.tracks.len() < usize::try_from(results.total.unwrap()).unwrap()
            } {}
        }
    }
}

