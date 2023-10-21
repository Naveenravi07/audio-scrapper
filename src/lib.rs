
use std::path:: PathBuf;
use rayon::prelude::*;
use rspotify::model:: PlaylistId;
use rspotify::prelude::BaseClient;
use rspotify::AuthCodeSpotify;
use youtube_dl::{SearchOptions, YoutubeDl};
use clap::{Parser, Subcommand};


#[derive(Debug, Parser)]
#[command(
    subcommand_value_name = "INPUT_METHOD",
    subcommand_help_heading = "Input Methods",
    disable_help_subcommand = true
    )]

pub struct Args {
    /// The directory to save downloaded files to.
    pub output_dir: PathBuf,
    #[command(subcommand)]
    pub method: InputMethod,
}

#[derive(Debug, Clone, Subcommand)]
pub enum InputMethod {
    /// A file containing a line-delimited list of tracks to search for.
    File { path: PathBuf },
    /// A Spotify playlist from your account.
    Spotify,
}
pub struct PlaylistTracks {
    pub tracks: Vec<String>,
    pub total: Option<u32>,
}


pub  async fn fetch_tracks_of_playlist(
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