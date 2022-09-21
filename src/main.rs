mod game;

fn main() {
    println!("\n\n\nWelcome to rustle: terminal wordle written in rust\n\n");
    println!("The rules are simple:\n\tGuess the random word in a maximum of six attempts to win the game!\n");
    let mut game = match game::Game::new("./dict.txt", "./repo.txt", "./user_data.txt") {
        Ok(game) => game,
        Err(mssg) => panic!("Failed to start game: {}", mssg)
    };
    game.play();
}
