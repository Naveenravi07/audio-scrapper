use std::time::Instant;
use std::{fs, thread};
use youtube_dl::{SearchOptions, YoutubeDl};

fn main() {
    let start = Instant::now();
    let file_path = String::from("/home/shastri/Music/music.txt");
    let content = fs::read_to_string(file_path);

    for music in content.unwrap().lines().map(|x| String::from(x)) {
        let threads = thread::spawn(move || {
            let options = SearchOptions::youtube(music.clone());
            let _audio = YoutubeDl::search_for(&options)
                .extract_audio(true)
                .output_template(music)
                .download_to("/home/shastri/Music2/")
                .unwrap();
        });
        threads.join().unwrap();
        let duration = start.elapsed();
        println!("Time elapsed in expensive_function() is: {:?}", duration);
    }
}

