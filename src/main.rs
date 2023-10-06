use audio_scrapper::{InputMethods, SpotifyHelpers};
use rspotify::{prelude::OAuthClient, scopes, AuthCodeSpotify, Credentials, OAuth};
use std::{fs, process};
use std::env;
use dotenvy::dotenv;


#[tokio::main]
async fn main() {
    let args: std::env::Args = std::env::args();

    let config = audio_scrapper::Config::build(args).unwrap_or_else(|message| {
        eprintln!("Problems in parsing arguments : {}", message);
        process::exit(1);
    });

    match config.method {
        InputMethods::File => {

            let content_raw = fs::read(config.input_file.as_ref().unwrap_or_else(||{
                eprintln!("Invalid input file");
                process::exit(1);
            }));

            let content = content_raw.unwrap_or_else(|err|{
                eprintln!("cannot read file");
                eprintln!("{}",err);
                process::exit(1);
            });

            let content_vec: Vec<String> = String::from_utf8(content)
                .unwrap()
                .split("\n")
                .map(|x| x.to_string())
                .collect();

            <AuthCodeSpotify as SpotifyHelpers>::downlaod_tracks_from_youtube(
                &content_vec,
                &config.output_dir,
            );
        }

        InputMethods::Spotify => {
            dotenv().expect(".env file not found");

            let client_id = env::var("client_id").unwrap_or_else(|err|{
                eprintln!("Cannot read client id from env file");
                eprintln!("{}",err);
                process::exit(1);
            });
            let client_secret = env::var("client_secret").unwrap_or_else(|err|{
                eprintln!("Cannot read client secret from env file");
                eprintln!("{}",err);
                process::exit(1);
            });

            let credentials = Credentials::new(
                &client_id,
                &client_secret,
            );

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
                let results = <AuthCodeSpotify as SpotifyHelpers>::fetch_tracks_of_playlist(
                    &spotify,
                    &playlisturl,
                    Some(offset),
                )
                .await;
                offset += 100;
                <AuthCodeSpotify as SpotifyHelpers>::downlaod_tracks_from_youtube(
                    &results.tracks,
                    &config.output_dir,
                );
                results.tracks.len() < usize::try_from(results.total.unwrap()).unwrap()
            } {}
        }
    }
}
