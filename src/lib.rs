#[derive(Clone,Copy,Debug)]
enum Cell {
    Nort,
    Cross,
    None
}

#[derive(Debug)]
enum MoveResult {
    Ongoing(Game),
    IllegalMove,
    WinFirstPlayer(String)
}

impl MoveResult {
    fn unwrap(self) -> Game {
        match self {
            MoveResult::Ongoing(game) => { game }
            _ => { panic!("Cannot play from this state") }
        }
    }
}

#[derive(Debug)]
struct Game {
    state: [Cell; 9],
    is_first_player_turn: bool
}

impl Game {
    pub fn new() -> Game {
            Game {
                state: [
                    Cell::None,
                    Cell::None,
                    Cell::None,
                    Cell::None,
                    Cell::None,
                    Cell::None,
                    Cell::None,
                    Cell::None,
                    Cell::None,
                ],
                is_first_player_turn: true
        }
    }

    pub fn display(&self) -> String {
        render_state(self.state)
    }

    pub fn make_move(&self, x: usize, y: usize) -> MoveResult {
        let mut state = self.state;
        if matches!(state[x + (y * 3)], Cell::None) {
            state[x + (y * 3)] = match self.is_first_player_turn {
                true => Cell::Nort,
                false => Cell::Cross
            };
            if is_win_state(state) {
                MoveResult::WinFirstPlayer(render_state(state))
            }
            else {
                MoveResult::Ongoing(Game {
                    state,
                    is_first_player_turn: !self.is_first_player_turn
                })
            }
        } else {
            MoveResult::IllegalMove
        }
    }
}

fn is_win_state(state: [Cell; 9]) -> bool {
    if matches!(state[0], Cell::Nort) && matches!(state[1], Cell::Nort) && matches!(state[2], Cell::Nort) {true}
    else {false}
}

fn render_state(state: [Cell; 9]) -> String {
    let mut result = String::new();

    for (i, cell) in state.iter().enumerate() {
        result.push_str(match cell {
            Cell::Nort => "O",
            Cell::Cross => "X",
            Cell::None => " "
        });

        if (i + 1) % 3 == 0 && i < 8 { result.push_str("\n")};
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_render_blank_board() {
        let game = Game::new();

        let actual = game.display();

        assert_eq!(
            concat!("   \n","   \n","   "),
            actual
        )
    }

    #[test]
    fn first_move_should_place_x_on_screen() {
        let mut game = Game::new();

        game = game.make_move(0,0).unwrap();

        let actual = game.display();
        assert_eq!(
            concat!("O  \n","   \n","   "),
            actual
        )
    }

    #[test]
    fn second_move_should_place_o_on_screen() {
        let mut game = Game::new();

        game = game.make_move(0,0).unwrap();
        game = game.make_move(1,0).unwrap();

        let actual = game.display();
        assert_eq!(
            concat!("OX \n","   \n","   "),
            actual
        )
    }

    #[test]
    fn move_on_second_row() {
        let mut game = Game::new();

        game = game.make_move(0,1).unwrap();

        let actual = game.display();
        assert_eq!(
            concat!("   \n","O  \n","   "),
            actual
        )
    }

    #[test]
    fn cannot_play_in_position_already_played_in() {
        let mut game = Game::new();

        game = game.make_move(0,0).unwrap();
        let result = game.make_move(0,0);

        assert!(matches!(result, MoveResult::IllegalMove))
    }

    #[test]
    fn win_first_player_horizontal() {
        let mut game = Game::new();

        game = game.make_move(0,0).unwrap();
        game = game.make_move(0,1).unwrap();
        game = game.make_move(1,0).unwrap();
        game = game.make_move(1,1).unwrap();
        let result = game.make_move(2,0);

        if let MoveResult::WinFirstPlayer(display) = result {
            assert_eq!(
                concat!("OOO\n","XX \n","   "),
                display
            )
        } else { panic!("Expected WinFirstPlayer, got {:?}",result) }
    }
}
