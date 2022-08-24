use rand::Rng;

#[derive(PartialEq)]
enum GameState {
    Title,
    Playing,
    Won
}

fn main() {
    let mut game_state = GameState::Title;
    let target_num = rand::thread_rng().gen_range(1..=100);
    let mut guesses = 0;

    println!("Number is between 1-100.");
    game_state = GameState::Playing;
    while GameState::Playing == game_state {
        let mut line: String = text_io::read!("{}\n");
        line.pop();
        match line.parse::<i32>() {
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
            Err(err) => {
                match err.kind() {
                    std::num::IntErrorKind::InvalidDigit => {
                        println!("Invalid input.");
                        continue;
                    },
                    _ => {
                        panic!();
                    }
                }
            }
        };
    }

    if game_state == GameState::Won {
        println!("You won in {} guesses. The number was {}.", guesses, target_num);
    }
}
