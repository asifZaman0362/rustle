use ansi_term::Color::{Green, White, Yellow};
use rand::Rng;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::path::Path;

struct Dictionary {
    words: Vec<String>
}

impl Dictionary {

    fn new(filename: &str) -> std::io::Result<Dictionary> {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);
        let mut words = vec![];
        for line in reader.lines() {
            let word = line?;
            words.push(word);
        }
        Ok(Dictionary { words })
    }

    fn find_word(&self, word: &str) -> bool {
        // binary search string
        let mut len = self.words.len();
        let mut left = 0;
        while left <= len {
            let mid = left + (len - left) / 2;
            if self.words[mid] == word {
                return true;
            } else if self.words[mid] < word.to_string() {
                left = mid + 1;
            } else {
                len = mid - 1;
            }
        }
        false
    }

}

enum GameResult {
    Win(String, u8),
    Lose(String),
}

fn get_word<P>(filename: P, line_number: usize) -> std::io::Result<String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    for (i, line) in reader.lines().enumerate() {
        if i == line_number {
            return line;
        }
    }
    Err(std::io::Error::new(
        io::ErrorKind::UnexpectedEof,
        "Reached end of file!",
    ))
}

fn check(input: &str, answer: &str) -> [i8; 5] {
    let upper = answer.to_uppercase();
    let mut matches = [0; 5];
    let mut mask: Vec<usize> = vec![];
    for (i, a) in input.chars().enumerate() {
        for (j, b) in upper.chars().enumerate() {
            if a == b && !(mask.contains(&j)) {
                matches[i] = (i == j).try_into().unwrap();
                mask.push(j);
                break;
            } else {
                matches[i] = -1;
            }
        }
    }
    matches
}

fn play_game() -> GameResult {
    let word_number = rand::thread_rng().gen_range(0..500);
    let word = get_word("./repo.txt", word_number).expect("Failed to get word from repository!");
    let dictionary = match Dictionary::new("./dictionary.txt") {
        Ok(dict) => dict,
        Err(err) => panic!("{}", err)
    };
    let mut input_buffer;
    let stdin = std::io::stdin();
    for i in 0..6 {
        loop {
            println!("Enter guess:");
            input_buffer = String::new();
            stdin
                .read_line(&mut input_buffer)
                .expect("Failed to read from stdin!");
            input_buffer = input_buffer.trim().to_owned();
            if input_buffer.chars().count() != 5 {
                println!("Invalid word! Must be exactly 5 letters long!");
                continue;
            } else {
                if !dictionary.find_word(&input_buffer) {
                    println!("I've never seen that word before!");
                    continue;
                }
                break;
            }
        }
        let matches = check(&input_buffer, &word);
        let mut correct = 0;
        for (i, result) in matches.into_iter().enumerate() {
            match result {
                1 => {
                    correct += 1;
                    print!(
                        "{}",
                        Green.paint(format!(
                            "[{}]",
                            input_buffer
                                .chars()
                                .nth(i as usize)
                                .expect("index out of bounds!")
                        ))
                    );
                }
                0 => print!(
                    "{}",
                    Yellow.paint(format!(
                        "[{}]",
                        input_buffer
                            .chars()
                            .nth(i as usize)
                            .expect("index out of bounds!")
                    ))
                ),
                -1 => print!(
                    "{}",
                    White.paint(format!(
                        "[{}]",
                        input_buffer
                            .chars()
                            .nth(i as usize)
                            .expect("index out of bounds!")
                    ))
                ),
                _ => panic!("Shouldn't have happened!"),
            };
        }
        println!("{}", correct);
        if correct == 5 {
            return GameResult::Win(word.to_owned(), i);
        }
    }
    GameResult::Lose(word.to_owned())
}


fn main() {
    let remarks = [
        "Genius!",
        "Brilliant",
        "Fantastic",
        "Nice",
        "Passable",
        "Whew!",
    ];
    println!("\n\n\nWelcome to rustle: terminal wordle written in rust\n\n");
    println!("The rules are simple:\n\tGuess the random word in a maximum of six attempts to win the game!\n");
    match play_game() {
        GameResult::Win(word, tries) => println!(
            "{}\nThe word was: {}\nYou solved it in: {} tries!\n",
            remarks[tries as usize], word, tries
        ),
        GameResult::Lose(word) => println!("Oops! The word was: {}\nBetter luck next time.", word),
    };
}
