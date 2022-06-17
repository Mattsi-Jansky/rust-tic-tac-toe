struct TicTacToeGame {

}

impl TicTacToeGame {
    pub fn new() -> TicTacToeGame {
        TicTacToeGame {}
    }

    pub fn display(&self) -> String {
        String::from(concat!("   \n","   \n","   "))
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
}
