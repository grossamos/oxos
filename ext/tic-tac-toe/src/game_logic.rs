use oxos_syscall::uart_send;

#[derive(Clone, Copy)]
#[derive(PartialEq, Eq)]
pub enum Player {
    Pl1,
    Pl2
}

pub struct GameState {
    game_field: [[Option<Player>; 3]; 3],
}

impl GameState {
    pub fn new() -> GameState {
        GameState { game_field: [[None; 3]; 3] }
    }

    pub fn check_for_winner(&self) -> Option<Player> {
        let row = self.get_winning_row();
        match row {
            None => None,
            Some(row) => {
                match self.game_field[row[0].0 as usize][row[0].1 as usize] {
                    Some(player) => Some(player),
                    None => panic!(),
                }
            }
        }
    }

    pub fn check_valid_play(&self, field: (u32, u32)) -> bool {
        self.game_field[field.0 as usize][field.1 as usize].is_none()
    }

    pub fn register_play(&mut self, field: (u32, u32), player: Player) {
        self.game_field[field.0 as usize][field.1 as usize] = Some(player);
    }

    pub fn get_winning_row(&self) -> Option<[(u32, u32); 3]> {
        let rows = [
            [(0, 0), (0, 1), (0, 2)],
            [(1, 0), (1, 1), (1, 2)],
            [(2, 0), (2, 1), (2, 2)],

            [(0, 0), (1, 0), (2, 0)],
            [(0, 1), (1, 1), (2, 1)],
            [(0, 2), (1, 2), (2, 2)],

            [(0, 0), (1, 1), (2, 2)],
            [(0, 2), (1, 1), (2, 0)],
        ];
        for row in rows {
            if self.check_if_wining_row(row) {
                return Some(row);
            }
        }
        None
    }

    fn check_if_wining_row(&self, row: [(u32, u32); 3]) -> bool {
        let mut winning = true;
        let winner = self.game_field[row[0].0 as usize][row[0].1 as usize];
        for (x, y) in row {
            winning = winning && (winner == self.game_field[x as usize][y as usize]);
        }
        winning && winner.is_some()
    }
}
