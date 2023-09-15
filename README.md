# Audio Scraper

A simple CLI tool that scrapes audio from YouTube if a match is found. This tool offers two main usage scenarios:

1) Download your entire Spotify playlist.
2) Download all songs by supplying their names through a text file.

## Prerequisites

1) Install Rust and Cargo on your system. Follow the instructions [here](https://doc.rust-lang.org/cargo/getting-started/installation.html).

2) Create a Spotify API app in the [Spotify Developer Dashboard](https://developer.spotify.com) and note down the "Client ID" and "Client Secret".

3) Create a .env file in the root dir of the project and paste the ``client_id`` and ``client_secret`` .

## Usage

### Usage with spotify
``` cargo run -- spotify ~/outputdir ```

### Usage with txt file 
``` cargo run -- file  ./input.txt  ~/outputdir```


