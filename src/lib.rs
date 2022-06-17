enum Cell {
    Nort,
    Cross,
    None
}

struct TicTacToeGame {
    state: [Cell; 9]
}

impl TicTacToeGame {
    pub fn new() -> TicTacToeGame {
            TicTacToeGame { state: [
                Cell::None,
                Cell::None,
                Cell::None,
                Cell::None,
                Cell::None,
                Cell::None,
                Cell::None,
                Cell::None,
                Cell::None,
            ]
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

    pub fn make_move(&mut self, p0: i32, p1: i32) {
        self.state[0] = Cell::Nort;
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
}
