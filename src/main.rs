use rspotify::{
    model::PlaylistId,
    prelude::{BaseClient, OAuthClient},
    scopes, AuthCodeSpotify, Credentials, OAuth,
};
use std::{fs, process, thread};
use youtube_dl::{SearchOptions, YoutubeDl};

#[tokio::main]
async fn main() {
    let args: std::env::Args = std::env::args();

    let config = audio_scrapper::Config::build(args).unwrap_or_else(|message| {
        eprintln!("Problems in parsing arguments : {}", message);
        process::exit(1);
    });

    let content = fs::read_to_string(&config.input_file).unwrap_or_else(|_err| {
        eprintln!("No file found at specified location");
        process::exit(1);
    });

    let credentials = Credentials::new(
        "7d4cca88e358409488db59c8dea2d3f9",
        "f375416f175b4931afae80e0641ca981",
    );

    let oauth = OAuth {
        redirect_uri: "http://localhost:42069".to_string(),
        scopes: scopes!("user-read-private user-read-email"),
        ..Default::default()
    };

    let spot_auth = AuthCodeSpotify::new(credentials, oauth);
    let url = spot_auth.get_authorize_url(false).unwrap();
    spot_auth.prompt_for_token(&url).await.unwrap();

    println!("Fetching your playlists ");

    let playlist = spot_auth
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

    println!("Downloading {}", playlist.items[num].name);

    let mut results: Vec<String> = Vec::new();
    let mut offset:u32 = 0 ;
    
    let songs_list = spot_auth
        .playlist_items_manual(
            PlaylistId::from_uri(&playlist.items[num].id.to_string()).unwrap(),
            None,
            None,
            Some(100),
            Some(offset),
        )
        .await
        .unwrap();

    for song in &songs_list.items {
        if let Some(track) = &song.track {
            match &track {
                rspotify::model::PlayableItem::Track(fulltrack) => {
                    results.push(fulltrack.name.clone());
                }
                _ => println!("deeznuts"),
            };
        }
    }
    println!("{:?}", songs_list.total);
    println!("{:?}", results.len());

    while results.len().clone() < usize::try_from(songs_list.total).unwrap() {
        offset+= 100;
        println!("Repeating...");
        let vaasu = spot_auth
            .playlist_items_manual(
                PlaylistId::from_uri(&playlist.items[num].id.to_string()).unwrap(),
                None,
                None,
                Some(100),
                Some(offset),
            )
            .await
            .unwrap();
    
        for song in &vaasu.items {
            if let Some(track) = &song.track {
                match &track {
                    rspotify::model::PlayableItem::Track(fulltrack) => {
                        results.push(fulltrack.name.clone());
                    }
                    _ => println!("deeznuts"),
                };
            }
        }
    }
    
    
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
