use std::io;
use rand::Rng;
use soloud::*;

const TONE_NAMES: [&str; 7] = ["a", "b", "c", "d", "e", "f", "g"];

fn main() {
    println!("Enter a tone.");

    let tone = TONE_NAMES.get(rand::thread_rng().gen_range(0..TONE_NAMES.len() - 1)).unwrap();

    println!("random note is {}", tone);

    let sl = Soloud::default().unwrap();

    let mut wav = audio::Wav::default();

    match tone.trim() {
        "a" => wav.load_mem(include_bytes!("../audio/a.wav")).unwrap(),
        "a#" | "bb" => wav.load_mem(include_bytes!("../audio/a.wav")).unwrap(), 
        "b" => wav.load_mem(include_bytes!("../audio/b.wav")).unwrap(),
        "c" => wav.load_mem(include_bytes!("../audio/c.wav")).unwrap(),
        "c#" | "db" => wav.load_mem(include_bytes!("../audio/a.wav")).unwrap(), 
        "d" => wav.load_mem(include_bytes!("../audio/d.wav")).unwrap(),
        "d#" | "eb" => wav.load_mem(include_bytes!("../audio/a.wav")).unwrap(), 
        "e" => wav.load_mem(include_bytes!("../audio/e.wav")).unwrap(),
        "f" => wav.load_mem(include_bytes!("../audio/f.wav")).unwrap(),
        "f#" | "gb" => wav.load_mem(include_bytes!("../audio/a.wav")).unwrap(), 
        "g" => wav.load_mem(include_bytes!("../audio/g.wav")).unwrap(),
        "g#" => wav.load_mem(include_bytes!("../audio/a.wav")).unwrap(), 
        &_ => todo!()
    }

    sl.play(&wav);

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("failed to read input");

    match guess.trim() == tone.trim() {
        true => println!("YAY!"),
        false => println!("NOOOO!")
    }

    println!("You've entered {}", tone);
}
