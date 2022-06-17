enum Cell {
    Nort,
    Cross,
    None
}

struct TicTacToeGame {
    state: [Cell; 9],
    is_first_player_turn: bool
}

impl TicTacToeGame {
    pub fn new() -> TicTacToeGame {
            TicTacToeGame {
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
        let mut result = String::new();

        for (i, cell) in self.state.iter().enumerate() {
            result.push_str(match cell {
                Cell::Nort => "O",
                Cell::Cross => "X",
                Cell::None => " "
            });

            if (i + 1) % 3 == 0 && i < 8 { result.push_str("\n")};
        }

        result
    }

    pub fn make_move(&mut self, x: usize, y: usize) {
        self.state[x] = match self.is_first_player_turn {
            true => Cell::Nort,
            false => Cell::Cross
        };
        self.is_first_player_turn = !self.is_first_player_turn
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_render_blank_board() {
        let game = TicTacToeGame::new();

        let actual = game.display();

        assert_eq!(
            concat!("   \n","   \n","   "),
            actual
        )
    }

    #[test]
    fn first_move_should_place_x_on_screen() {
        let mut game = TicTacToeGame::new();

        game.make_move(0,0);

        let actual = game.display();
        assert_eq!(
            concat!("O  \n","   \n","   "),
            actual
        )
    }

    #[test]
    fn second_move_should_place_o_on_screen() {
        let mut game = TicTacToeGame::new();

        game.make_move(0,0);
        game.make_move(1,0);


        let actual = game.display();
        assert_eq!(
            concat!("OX \n","   \n","   "),
            actual
        )
    }
}
