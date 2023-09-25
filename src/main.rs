use rand::Rng;
use soloud::*;
use std::io;

// TODO: create a struct/class that includes a sound and have a "play" function to it to play it.

const TONE_NAMES: [&str; 7] = ["a", "b", "c", "d", "e", "f", "g"];
const MAX_SCORE: i32 = 10;

fn main() {
    let sl = Soloud::default().unwrap();

    let mut wav = audio::Wav::default();

    let mut score = 0;

    let mut is_redo = false;

    let mut tone = "";

    while score <= MAX_SCORE {
        println!("Enter a tone. Submit 'r' to hear the tone again.");

        if !is_redo {
            tone = TONE_NAMES
                .get(rand::thread_rng().gen_range(0..TONE_NAMES.len() - 1))
                .unwrap();

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
                &_ => todo!(),
            };
        }


        sl.play(&wav);

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read input");

        if guess.trim() == "r" {
            is_redo = true;

            sl.play(&wav);
        } else {
            is_redo = false;

            match guess.trim() == tone.trim() {
                true => {
                    score += 1;
                    println!("Correct!! Your score is {} of {}.", score, MAX_SCORE);
                }
                false => {
                    println!("WRONG! You entered {} but the answer was {}.", guess, tone);
                }
            }
        }
    }
}
