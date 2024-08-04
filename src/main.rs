use std::io;

fn main() {
    let mut game_running = true;
    let mut turn_counter = 0;
    let player = ['X', 'O'];
    let mut player_input = ' ';
    let mut board_positions = [["1", "2", "3"],["4", "5", "6"],["7", "8", "9"]];

    while game_running == true {
        turn_counter += 1;
        println!("Turn {}", turn_counter);
        println!("Current Player: {}", player[turn_counter % player.len()]);
        println!("----------------------------");
        display_board(&mut board_positions);
        println!("----------------------------");
        println!("Enter number: ");
        io::stdin().read_line(&mut player_input).expect("Invalid Input");
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