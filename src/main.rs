#[macro_use] extern crate rocket;
use game::Game;
use rocket::State;

mod game;

#[get("/")]
async fn index(game_state: &State<game::Game>) -> String {
    "Hello, World".to_owned()
}

#[get("/guess/<word>")]
async fn guess(game_state: &State<game::Game>, word: String) -> String {
    match game_state.validate_input(&word) {
        Ok(_) => {
            let guess_result = game_state.check(&word);
            let mut correct = 0;
            for i in guess_result.into_iter() {
                if i == 1 {
                    correct += 1;
                }
            }
            if correct == 5 {
                return "Correct!".to_owned();
            } else {
                return "Incorrect!".to_owned();
            }
        },
        Err(mssg) => mssg.to_owned()
    }
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let mut game = match game::Game::new("./dict.txt", "./repo.txt", "./user_data.txt") {
        Ok(game) => game,
        Err(mssg) => panic!("Failed to start game: {}", mssg)
    };
    println!("random word: {}", game.get_random_word());
    let _rocket = rocket::build()
        .manage(game)
        .mount("/", routes![index, guess])
        .launch()
        .await?;
    Ok(())
}
