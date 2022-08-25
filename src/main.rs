use rand::Rng;
use serde::{Serialize, Deserialize};

#[derive(PartialEq)]
enum GameState {
    Title,
    Playing,
    Won,
}

#[derive(Serialize, Deserialize, Debug)]
struct Stats {
    guesses: i32
}

fn main() {
    let mut game_state = GameState::Title;
    let lower_number = 1;
    let higher_number = 100;
    let target_num = rand::thread_rng().gen_range(lower_number..=higher_number);
    let mut guesses = 0;
    let mut stats: Stats = read_storage();

    println!("Number is between {}-{}.", lower_number, higher_number);
    game_state = GameState::Playing;
    while GameState::Playing == game_state {
        let line: String = text_io::read!("{}\n");
        match line.trim().parse::<i32>() {
            Ok(val) => {
                if val == target_num {
                    game_state = GameState::Won;
                    break;
                } else if target_num > val {
                    println!("Greater than {}.", val);
                } else if target_num < val {
                    println!("Less than {}.", val);
                }
                guesses += 1;
            }
            Err(_) => {
                println!("Invalid input.");
                continue;
            }
        }
    }
    stats.guesses += guesses;

    if game_state == GameState::Won {
        println!(
            "You won in {} guesses. The number was {}.",
            guesses, target_num
        );
    }
    println!("You've made {} guesses so far.", stats.guesses);

    write_storage(&stats);
}

fn write_storage(stats: &Stats) {
    let serialized = serde_json::to_string(stats).unwrap();
    std::fs::write("stats.json", serialized).expect("Unable to write to file.");
}

fn read_storage() -> Stats {
    let serialized = std::fs::read_to_string("stats.json").expect("Unable to read file.");
    serde_json::from_str(&serialized).unwrap()
}
