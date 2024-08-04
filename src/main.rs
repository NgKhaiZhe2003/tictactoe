use std::{collections::btree_map::Values, default, io};

fn main() {
    let mut game_running = true;
    let mut turn_counter = 0;
    let player = ['X', 'O'];
    
    let mut board_positions = [["1", "2", "3"],["4", "5", "6"],["7", "8", "9"]];

    while game_running == true {
        turn_counter += 1;

        // display ui
        println!("Turn {}", turn_counter);
        println!("Current Player: {}", player[turn_counter % player.len()]);
        println!("----------------------------");
        display_board(&mut board_positions);
        println!("----------------------------");
        println!("Enter number: ");

        // return parsed user input or 0
        let parsed_input = get_player_input();
        //println!("{}", parsed_input);

        // check if 3 in a row has occured or not

        // if occured end game and display results
        end_game(&mut game_running);
    }
}

fn end_game(game_running: &mut bool) {
    *game_running = false;
}

fn display_board(pos: &mut [[&str; 3]; 3]) {
    println!("     |     |     ");
    println!("  {}  |  {}  |  {}   ", pos[0][0], pos[0][1], pos[0][2]);
    println!("_____|_____|_____");
    println!("     |     |     ");
    println!("  {}  |  {}  |  {}   ", pos[1][0], pos[1][1], pos[1][2]);
    println!("_____|_____|_____");
    println!("     |     |     ");
    println!("  {}  |  {}  |  {}   ", pos[2][0], pos[2][1], pos[2][2]);
    println!("     |     |     ");
}

fn get_player_input() -> u8 {
    let mut player_input = String::new();
    let mut player_input_int: u8 = 0;

    io::stdin()
        .read_line(&mut player_input)
        .expect("Invalid Input");
    match player_input.trim().parse() {
        Ok(value) => {
            player_input_int = value;
        }
        Err(_) => {
            println!("Found non-number character. Please retry input.");
        }
    }

    // filter out numbers outside 1-9, return 0 for invalid input
    match player_input_int {
        1..=9 => return player_input_int,
        _ => 0,
    }

}