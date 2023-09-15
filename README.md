# Audio Scraper

A simple CLI tool that scrapes audio from YouTube if a match is found. This tool offers two main usage scenarios:

1) Download your entire Spotify playlist.
2) Download all songs by supplying their names through a text file.

## Before You Download

Install Rust and Cargo on your system. You can follow the instructions [here](https://doc.rust-lang.org/cargo/getting-started/installation.html).

## Usage

### Usage with spotify
``` cargo run -- spotify ~/outputdir ```

### Usage with txt file 
``` cargo run -- file  ./input.txt  ~/outputdir```


