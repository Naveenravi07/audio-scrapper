
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use rspotify::model::PlaylistId;
use rspotify::prelude::BaseClient;
use rspotify::AuthCodeSpotify;
use youtube_dl::{SearchOptions, YoutubeDl};

#[derive(Debug)]
pub struct Config {
    pub input_file: Option<String>,
    pub output_dir: String,
    pub method: InputMethods,
}

#[derive(Debug)]
pub enum InputMethods {
    File,
    Spotify,
}

impl Config {
    pub fn build(mut args: std::env::Args) -> Result<Self, String> {
        if args.len() < 3 {
            return Err(String::from("Not Enough Args"));
        }
        args.next();

        let input_method_str = match args.next() {
            Some(mehod_str) => mehod_str,
            None => {
                eprintln!("Invalid Input Method Selected");
                eprintln!("1st argument -> File | Spotify");
                return Err("Input Method missing in args".to_string());
            }
        };

        let input_method = match &*input_method_str {
            "file" => InputMethods::File,
            "spotify" => InputMethods::Spotify,
            _ => {
                eprintln!("Invalid Input Method Selected");
                eprintln!("1st argument -> File | Spotify");
                return Err("Invalid Input Method".to_string());
            }
        };

        let input_file = if let InputMethods::File = input_method {
            match args.next() {
                Some(name) => Some(name),
                None => return Err(String::from("Input file missing in args")),
            }
        } else {
            None
        };

        let output_dir = match args.next() {
            Some(name) => name,
            None => return Err(String::from("Output dir missing in args")),
        };

        Ok(Config {
            input_file,
            output_dir,
            method: input_method,
        })
    }
}


pub struct PlaylistTracks {
    pub tracks: Vec<String>,
    pub total: Option<u32>,
}

#[async_trait::async_trait]
pub trait SpotifyHelpers {
    async fn fetch_tracks_of_playlist(
        spotify: &AuthCodeSpotify,
        playlisturl: &String,
        offset: Option<u32>,
    ) -> PlaylistTracks;

    fn downlaod_tracks_from_youtube(tracks: &Vec<String>, output_dir: &String);
}

#[async_trait::async_trait]
impl SpotifyHelpers for AuthCodeSpotify {
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

    fn downlaod_tracks_from_youtube(tracks: &Vec<String>, output_dir: &String) {
        tracks.par_iter().for_each(|music|{ 
            let options = SearchOptions::youtube(music);
            let audio = YoutubeDl::search_for(&options)
                .output_template(&format!("{}.m4a",music))
                .format("140")
                .download_to(output_dir);
            match audio {
                Ok(_) => println!("{} Download Successfull", music),
                Err(err) => println!("Err Downloading {} from youtube,{:?}", music,err),
            }
        })
    }
}
