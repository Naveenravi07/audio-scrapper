use audio_scrapper::{InputMethods, SpotifyHelpers};
use rspotify::{prelude::OAuthClient, scopes, AuthCodeSpotify, Credentials, OAuth};
use std::time::Instant;
use std::{fs, process, thread};
use youtube_dl::{SearchOptions, YoutubeDl};


#[tokio::main]
async fn main() {
    let args: std::env::Args = std::env::args();

    let config = audio_scrapper::Config::build(args).unwrap_or_else(|message| {
        eprintln!("Problems in parsing arguments : {}", message);
        process::exit(1);
    });

    println!("{:?}", config);

    match config.method {
        InputMethods::File => {
            let content =
                fs::read_to_string(&config.input_file.as_ref().unwrap()).unwrap_or_else(|_err| {
                    eprintln!("No file found at specified location");
                    process::exit(1);
                });

            for music in content.lines().map(|x| String::from(x)) {
                let folder_location = config.output_dir.clone();
                let threads = thread::spawn(move || {
                    let options = SearchOptions::youtube(&music);
                    let audio = YoutubeDl::search_for(&options)
                        .extract_audio(true)
                        .output_template(&music)
                        .download_to(folder_location);
                    match audio {
                        Ok(_) => println!("{} Download Successfull", music),
                        Err(_) => println!("Err Downloading {} from youtube", music),
                    }
                });
                threads.join().unwrap();
            }
        }

        InputMethods::Spotify => {
            let start = Instant::now();

            let credentials = Credentials::new(
                "7d4cca88e358409488db59c8dea2d3f9",
                "e63d6a668a5d43a08c095d8cc8d7b6cb",
            );

            let oauth = OAuth {
                redirect_uri: "http://localhost:42069".to_string(),
                scopes: scopes!("user-read-private user-read-email"),
                ..Default::default()
            };

            let spotify = AuthCodeSpotify::new(credentials, oauth);
            let url = spotify.get_authorize_url(false).unwrap();
            spotify.prompt_for_token(&url).await.unwrap();

            println!("Fetching your playlists ");
            let playlist = spotify
                .current_user_playlists_manual(None, None)
                .await
                .unwrap();

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
            let duration = start.elapsed();
            println!("Time elapsed in expensive_function() is: {:?}", duration);
        }
    }
}
