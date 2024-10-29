use rand::Rng;
use rodio::{source::Source, Decoder, OutputStream};
use std::fs::{self, File};
use std::io::{self, BufReader};
use std::path::PathBuf;
use std::process::exit;
use std::thread;

use clap::Parser;

#[derive(Parser)]
struct Args {
    path: String,

    #[arg(default_value_t = 60)]
    seconds: u32,

    #[arg(long, default_value_t = false)]
    log: bool,
}

fn main() {
    let args = Args::parse();

    let seconds = args.seconds;

    println!("{}", seconds);
    let read_dir = match fs::read_dir(&args.path) {
        Ok(d) => d,
        Err(_) => {
            print!("path not found");
            exit(0)
        }
    };
    let mut lists: Vec<PathBuf> = Vec::new();
    for music in read_dir {
        let music = music.unwrap().path();

        if music.to_str().unwrap().to_string().ends_with(".mp3") {
            lists.push(music)
        }
    }
    if lists.len() == 0 {
        print!("mp3 not found");
        exit(0)
    }

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    thread::spawn(move || loop {
        let rand_sound = rand::thread_rng().gen_range(0..lists.len());
        let file = BufReader::new(File::open(&lists[rand_sound]).unwrap());
        let source = Decoder::new(file).unwrap();
        let r = rand::thread_rng().gen_range(1..=seconds);
        let _ = stream_handle.play_raw(source.convert_samples());
        if args.log {
            println!(
                "playing {}",
                &lists[rand_sound].as_os_str().to_str().unwrap().to_string()
            );
        }

        std::thread::sleep(std::time::Duration::from_secs(r as u64))
    });
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("what?");
        break;
    }
}
