use rand::Rng;
use rodio::{source::Source, Decoder, OutputStream};
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufReader};
use std::process::exit;
use std::{env, thread};

fn main() {
    let argv: Vec<String> = env::args().collect();
    if argv.len() == 1 {
        print!("type path as argument...");
        exit(0);
    }
    let mut seconds: u32 = match argv.len() != 3 {
        true => 60,
        false => match argv[2].trim().parse::<u32>() {
            Ok(r) => r,
            Err(_) => 60,
        },
    };
    println!("{}", seconds);
    let read_dir = match fs::read_dir(&argv[1]) {
        Ok(d) => d,
        Err(_) => {
            print!("path not found");
            exit(0)
        }
    };
    let mut lists: Vec<String> = Vec::new();
    for music in read_dir {
        let asd = music
            .unwrap()
            .path()
            .as_os_str()
            .to_str()
            .unwrap()
            .to_string();
        if asd.ends_with(".mp3") {
            lists.push(asd)
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
        println!("playing {}", &lists[rand_sound].to_string());

        std::thread::sleep(std::time::Duration::from_secs(r as u64))
    });
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("what?");
        break;
    }
}
