use rand::Rng;

#[derive(PartialEq)]
enum GameState {
    Title,
    Playing,
    Won
}

fn main() {
    let mut game_state = GameState::Title;
    let lower_number = 1;
    let higher_number = 100;
    let target_num = rand::thread_rng().gen_range(lower_number..=higher_number);
    let mut guesses = 0;

    println!("Number is between {}-{}.", lower_number, higher_number);
    game_state = GameState::Playing;
    while GameState::Playing == game_state {
        let line: String = text_io::read!("{}\n");
        match line.trim().parse::<i32>() {
            Ok(val) => {
                if val == target_num {
                    game_state = GameState::Won;
                    break;
                }
                else if target_num > val {
                    println!("Greater than {}.", val);
                }
                else if target_num < val {
                    println!("Less than {}.", val);
                }
                guesses += 1;
            },
            Err(_) => {
                println!("Invalid input.");
                continue;
            }
        }
    };

    if game_state == GameState::Won {
        println!("You won in {} guesses. The number was {}.", guesses, target_num);
    }
}
