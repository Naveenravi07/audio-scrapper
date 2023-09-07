use audio_scrapper::{Config, InputMethods};
use rspotify::{
    model::PlaylistId,
    prelude::{BaseClient, OAuthClient},
    scopes, AuthCodeSpotify, Credentials, OAuth,
};
use std::{fs, process, thread};
use youtube_dl::{SearchOptions, YoutubeDl};

pub struct PlaylistTracks {
    tracks: Vec<String>,
    total: Option<u32>,
}

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
            let content = fs::read_to_string(&config.input_file.unwrap()).unwrap_or_else(|_err| {
                eprintln!("No file found at specified location");
                process::exit(1);
            });

            for music in content.lines().map(|x| String::from(x)) {
                let output_dir_clone = config.output_dir.clone();
                let threads = thread::spawn(move || {
                    let options = SearchOptions::youtube(&music);
                    let audio = YoutubeDl::search_for(&options)
                        .extract_audio(true)
                        .output_template(&music)
                        .download_to(output_dir_clone);
                    match audio {
                        Ok(_) => println!("{} Download Successfull", music),
                        Err(_) => println!("Err Downloading {} from youtube", music),
                    }
                });
                threads.join().unwrap();
            }
        }

        InputMethods::Spotify => {}
    }

    let credentials = Credentials::new(
        "7d4cca88e358409488db59c8dea2d3f9",
        "f375416f175b4931afae80e0641ca981",
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
        
        let results = fetch_tracks_of_playlist(&spotify, &playlisturl, Some(offset)).await;
        offset += 100;
        downlaod_tracks_from_youtube(&results.tracks,config.output_dir.clone());

        results.tracks.len().clone() < usize::try_from(results.total.unwrap()).unwrap()
    } {}
}
async fn fetch_tracks_of_playlist(
    spotify: &AuthCodeSpotify,
    playlisturl: &String,
    offset: Option<u32>,
) -> PlaylistTracks {
    let mut results: Vec<String> = Vec::new();
    let songs = spotify
        .playlist_items_manual(
            PlaylistId::from_uri(&playlisturl).unwrap(),
            None,
            None,
            Some(100),
            offset,
        )
        .await
        .unwrap();

    for song in songs.items {
        if let Some(track) = &song.track {
            match &track {
                rspotify::model::PlayableItem::Track(fulltrack) => {
                    results.push(fulltrack.name.clone());
                }
                _ => println!("deeznuts"),
            };
        }
    }
    PlaylistTracks {
        tracks: results,
        total: Some(songs.total),
    }
}

fn downlaod_tracks_from_youtube(tracks: &Vec<String>, output_dir:String) {
    for music in tracks {
        println!("{}",output_dir);
        let music = music.clone();
        let output_dir = output_dir.clone();
        let threads = thread::spawn(move || {
            let options = SearchOptions::youtube(&music);
            let audio = YoutubeDl::search_for(&options)
                .extract_audio(true)
                .output_template(&music)
                .download_to(output_dir);
            match audio {
                Ok(_) => println!("{} Download Successfull", music),
                Err(_) => println!("Err Downloading {} from youtube", music),
            }
        });
        threads.join().unwrap();
    }
}
