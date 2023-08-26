use std::fs;
use youtube_dl::{SearchOptions, YoutubeDl};

fn main() {
    let file_path = String::from("/home/shastri/Music/music.txt");
    let content = fs::read_to_string(file_path);

    for music in content.unwrap().lines() {
        println!("Music is {}", music);
        let options = SearchOptions::youtube(music);
        let audio = YoutubeDl::search_for(&options)
            .extract_audio(true)
            .output_template(music)
            .download_to("/home/shastri/Music2/")
            .unwrap();
    };
}
