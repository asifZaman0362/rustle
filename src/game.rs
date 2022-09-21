use ansi_term::{ Style, Color::{Green, Yellow, Fixed}};
use rand::Rng;
use std::fs::File;
use std::io::{prelude::*, BufReader, BufWriter};

struct UserData<'a> {
    guess_distribution: Vec<(u8, usize)>,
    path: &'a str
}

impl<'a> UserData<'a> {
    fn new(path: &str) -> UserData {
        let guess_distribution: Vec<(u8, usize)> = vec![ (1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0) ];
        let path = path;
        UserData { guess_distribution, path }
    }

    fn load(filepath: &str) -> std::io::Result<UserData> {
        let datafile = File::open(&filepath)?;
        let path = filepath;
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
        let datafile = File::create(self.path)?;
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

pub struct Game<'a> {
    dictionary: Vec<String>,
    word_pool: Vec<String>,
    data: UserData<'a>
}

impl<'a> Game<'a> {
    pub fn new(dictionary_path: &str, word_pool_path: &str, data_path: &'a str) -> std::io::Result<Game<'a>> {
        let data = match UserData::load(data_path) {
            Ok(data) => data,
            Err(_) => {
                println!("Failed to load user data!");
                UserData::new(data_path)
            }
        };
        let dictionary = Game::load_dictionary(dictionary_path)?;
        let word_pool = Game::load_words(word_pool_path)?;
        Ok(Game {
            dictionary,
            word_pool,
            data
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

    fn get_random_word(&self) -> &String {
        let mut rng = rand::thread_rng();
        let random_word_number = rng.gen_range(0..self.word_pool.len());
        &self.word_pool[random_word_number as usize]
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

    fn get_input(&self) -> String {
        let mut string = String::new();
        let stdin = std::io::stdin();
        stdin.read_line(&mut string).expect("Failed to read from standard input!");
        string.trim().to_owned()
    }

    fn validate_input(&self, guess: String) -> Result<String, &'static str> {
        if guess.len() > 5 {
            Err("Your guess must be exactly 5 letters long!")
        } else {
            match self.dictionary.binary_search(&guess) {
                Ok(_) => Ok(guess),
                Err(_) => Err("I've never seen that word before!")
            }
        }
    }

    fn start_game(&self) -> GameResult {
        let answer = self.get_random_word();
        for i in 0..6 {
            let guess = loop {
                println!("\nEnter your guess:");
                match self.validate_input(self.get_input()) {
                    Ok(guess) => {
                        break guess
                    }
                    Err(message) => {
                        println!("{}", message);
                        continue;
                    }
                };
            };
            let matches = Game::check(&guess, answer);
            let mut correct = 0;
            let col_white = Fixed(255);
            let col_gray_bg = Fixed(242);
            let text_style_correct = Style::new().bold().on(Green).fg(col_white);
            let text_style_misplaced = Style::new().bold().on(Yellow).fg(col_white);
            let text_style_invalid = Style::new().bold().on(col_gray_bg).fg(col_white);
            for (i, result) in matches.into_iter().enumerate() {
                match result {
                    1 => {
                        correct += 1;
                        print!(
                            "{}",
                            text_style_correct.paint(format!(
                                "[{}]",
                                guess
                                    .chars()
                                    .nth(i as usize)
                                    .expect("index out of bounds!")
                            ))
                        );
                    }
                    0 => print!(
                        "{}",
                        text_style_misplaced.paint(format!(
                            "[{}]",
                            guess
                                .chars()
                                .nth(i as usize)
                                .expect("index out of bounds!")
                        ))
                    ),
                    -1 => print!(
                        "{}",
                        text_style_invalid.paint(format!(
                            "[{}]",
                            guess
                                .chars()
                                .nth(i as usize)
                                .expect("index out of bounds!")
                        ))
                    ),
                    _ => panic!("Shouldn't have happened!"),
                };
            }
            if correct == 5 {
                return GameResult::Win(answer.to_owned(), i);
            }
        }
        GameResult::Lose(answer.to_owned())
    }

    pub fn play(&mut self) {
        let remarks = vec![ "Genius", "Marvelous", "Amazing", "Nice", "Passable", "Whew" ];
        loop {
            match self.start_game() {
                GameResult::Win(_, tries) => {
                    println!("{}, you win!", remarks[tries as usize]);
                    self.data.guess_distribution[tries as usize].1 += 1;
                    match self.data.dump() {
                        Ok(_) => {},
                        Err(err) => panic!("Failed to save user data: {}", err)
                    };
                },
                GameResult::Lose(word) => {
                    println!("Oof! The word was {}. Better luck next time!", word);
                }
            }
            println!("Would you like to play again? (y/Y)");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("failed to read standard input!");
            match input.trim() {
                "y" | "Y" => continue,
                _ => break,
            };
        }
    }
}
