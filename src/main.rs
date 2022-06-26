use std::io;
use tic_tac_toe::{Game, MoveResult};

fn main() {
    let mut game = Game::new();
    let mut result = MoveResult::Ongoing(Default::default());
    println!("{}", game.render());

    while !matches!(result, MoveResult::WinFirstPlayer(_))
        && !matches!(result, MoveResult::WinSecondPlayer(_)) {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                input = input.lines().last().unwrap().to_string();
                if n != 4 || !input.contains(",") { println!("Error, bad command. Use format `x,y` eg `0,0` (zero-indexed)")}
                else {
                    let mut input_chars = input.chars();
                    let x = input_chars.next().unwrap() as usize - 0x30;
                    input_chars.next();
                    let y = input_chars.next().unwrap() as usize - 0x30;
                    result = game.make_move(x,y);
                }
            }
            Err(error) => println!("error: {}. Try again", error),
        }

        if let MoveResult::Ongoing(new_game) = result {
            game = new_game;
        } else {
            println!("Illegal move. Use format `x,y` eg `0,0` (zero-indexed)")
        }

        println!("{}", game.render());
    }

    print!("Game over. ");
    match result {
        MoveResult::WinFirstPlayer(_) => { println!("First player (O) wins!") }
        MoveResult::WinSecondPlayer(_) => { println!("Second player (X) wins! ")}
        MoveResult::Draw(_) => { println!("Draw!") }
        MoveResult::IllegalMove | MoveResult::Ongoing(_) => { panic!("Impossible to reach here") }
    }
}
