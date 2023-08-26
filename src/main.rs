use std::time::Instant;
use std::{fs, process, thread};
use youtube_dl::{SearchOptions, YoutubeDl};

fn main() {
    let start = Instant::now();
    let args: std::env::Args = std::env::args();
    let config = audio_scrapper::Config::build(args).unwrap_or_else(|message| {
        eprintln!("Problems in parsing arguments : {}", message);
        process::exit(1);
    });

    let content = fs::read_to_string(&config.input_file).unwrap_or_else(|_err|{
        eprintln!("No file found at specified location");
        process::exit(1);
    });

    for music in content.lines().map(|x| String::from(x)) {
        let output_dir_clone = config.output_dir.clone();
        let threads = thread::spawn(move || {
            let options = SearchOptions::youtube(music.clone());
            let _audio = YoutubeDl::search_for(&options)
                .extract_audio(true)
                .output_template(music)
                .download_to(output_dir_clone)
                .unwrap();
        });
        threads.join().unwrap();
        let duration = start.elapsed();
        println!("Time elapsed in expensive_function() is: {:?}", duration);
    }
}
