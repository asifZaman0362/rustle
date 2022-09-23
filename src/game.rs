use ansi_term::{ Style, Color::{Green, Yellow, Fixed}};
use rand::Rng;
use std::fs::File;
use std::io::{prelude::*, BufReader, BufWriter};
use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize)]
pub struct Guess {
    pub letters: [(char, u8); 5]
}

struct UserData {
    guess_distribution: Vec<(u8, usize)>,
    path: String
}

impl UserData {
    fn new(path: &str) -> UserData {
        let guess_distribution: Vec<(u8, usize)> = vec![ (1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0) ];
        let path = path.to_owned();
        UserData { guess_distribution, path }
    }

    fn load(filepath: &str) -> std::io::Result<UserData> {
        let datafile = File::open(&filepath)?;
        let path = filepath.to_owned();
        let mut guess_distribution = vec![];
        let reader = BufReader::new(datafile);
        for line in reader.lines() {
            let data = line?;
            match data.split_once(":") {
                Some((tries, freq)) => {
                    let tr = tries.parse::<u8>().unwrap();
                    let fr = freq.parse::<usize>().unwrap();
                    guess_distribution.push((tr, fr));
                }
                None => {}
            };
        }
        Ok(UserData { guess_distribution, path })
    }

    fn dump(&self) -> std::io::Result<usize> {
        let datafile = File::create(&self.path)?;
        let mut writer = BufWriter::new(datafile);
        let mut bytes: usize = 0;
        for (tries, freq) in &self.guess_distribution {
            let string = format!("{}:{}\n", tries, freq);
            bytes += writer.write(string.as_bytes())?;
        }
        Ok(bytes)
    }
}

enum GameResult {
    Win(String, u8),
    Lose(String),
}

pub struct Game {
    dictionary: Vec<String>,
    word_pool: Vec<String>,
    data: UserData,
    chosen: String
}

impl Game {
    pub fn new(dictionary_path: &str, word_pool_path: &str, data_path: &str) -> std::io::Result<Game> {
        let data = match UserData::load(data_path) {
            Ok(data) => data,
            Err(_) => {
                println!("Failed to load user data!");
                UserData::new(data_path)
            }
        };
        let dictionary = Game::load_dictionary(dictionary_path)?;
        let word_pool = Game::load_words(word_pool_path)?;
        let chosen = "".to_owned();
        Ok(Game {
            dictionary, word_pool, data, chosen
        })
    }

    fn load_dictionary(dictionary_path: &str) -> std::io::Result<Vec<String>> {
        let file = File::open(dictionary_path)?;
        let reader = BufReader::new(file);
        let mut words = vec![];
        for line in reader.lines() {
            let word = line?;
            words.push(word);
        }
        Ok(words)
    }

    fn load_words(word_pool_path: &str) -> std::io::Result<Vec<String>> {
        let data = File::open(word_pool_path)?;
        let mut words = vec![];
        let reader = BufReader::new(data);
        for line in reader.lines() {
            words.push(line?);
        }
        Ok(words)
    }

    pub fn get_random_word(&mut self) -> &String {
        let mut rng = rand::thread_rng();
        let random_word_number = rng.gen_range(0..self.word_pool.len());
        self.chosen = self.word_pool[random_word_number as usize].as_str().to_owned();
        &self.chosen
    }

    pub fn check(&self, input: &str) -> Guess {
        let mut guess_arr : [char; 5] = ['0'; 5];
        let upper = self.chosen.to_uppercase();
        let mut matches = [0; 5];
        let mut mask: Vec<usize> = vec![];
        for (i, a) in input.chars().enumerate() {
            for (j, b) in upper.chars().enumerate() {
                if a == b && !(mask.contains(&j)) {
                    matches[i] = (i == j).try_into().unwrap();
                    mask.push(j);
                    break;
                } else {
                    matches[i] = 2;
                }
            }
        }
        let mut guesses = Guess {
            letters: [('0', 0); 5]
        };
        for i in 0..5 {
            guesses.letters[i] = (input.chars().nth(i).unwrap(), matches[i]);
        };
        guesses
    }

    fn get_input(&self) -> String {
        let mut string = String::new();
        let stdin = std::io::stdin();
        stdin.read_line(&mut string).expect("Failed to read from standard input!");
        string.trim().to_owned()
    }

    pub fn validate_input(&self, guess: &String) -> Result<String, &'static str> {
        if guess.len() > 5 {
            Err("Your guess must be exactly 5 letters long!")
        } else {
            match self.dictionary.binary_search(&guess) {
                Ok(_) => Ok(guess.clone()),
                Err(_) => Err("I've never seen that word before!")
            }
        }
    }

}
