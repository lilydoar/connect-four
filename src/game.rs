use crate::randomizer::Randomizer;

pub const BOARD_WIDTH: usize = 7;
pub const BOARD_HEIGHT: usize = 6;

pub struct Game {
    board: Board,
    state: State,
    win_orientations: [WinOrientation; 4],
}

pub type Board = [Tile; BOARD_WIDTH * BOARD_HEIGHT];

#[derive(Copy, Clone, PartialEq)]
pub enum Tile {
    Empty,
    Full(Player),
}

pub enum State {
    Turn(Player),
    Win(Player),
    Tie,
}

#[derive(Copy, Clone, PartialEq)]
pub enum Player {
    Player1,
    Player2,
}

pub enum Action {
    PlaceTile(Column),
    Restart(Player),
}

pub struct Column(usize);

struct WinOrientation {
    offsets: [usize; 4],
    // The range of rows and columns the orientation can be found in
    row_range: std::ops::Range<usize>,
    col_range: std::ops::Range<usize>,
}

impl Game {
    pub fn new(first_player: &Player) -> Self {
        let horizontal = WinOrientation {
            offsets: [0, 1, 2, 3],
            row_range: 0..BOARD_HEIGHT,
            col_range: 0..BOARD_WIDTH - 3,
        };

        let vertical = WinOrientation {
            offsets: [0, BOARD_WIDTH, BOARD_WIDTH * 2, BOARD_WIDTH * 3],
            row_range: 0..BOARD_HEIGHT - 3,
            col_range: 0..BOARD_WIDTH,
        };

        let diagonal_bottom_left = WinOrientation {
            offsets: [0, BOARD_WIDTH + 1, BOARD_WIDTH * 2 + 2, BOARD_WIDTH * 3 + 3],
            row_range: 0..BOARD_HEIGHT - 3,
            col_range: 0..BOARD_WIDTH - 3,
        };

        let diagonal_bottom_right = WinOrientation {
            offsets: [0, BOARD_WIDTH - 1, BOARD_WIDTH * 2 - 2, BOARD_WIDTH * 3 - 3],
            row_range: 0..BOARD_HEIGHT - 3,
            col_range: 3..BOARD_WIDTH,
        };

        Self {
            board: [Tile::Empty; BOARD_WIDTH * BOARD_HEIGHT],
            state: State::Turn(*first_player),
            win_orientations: [
                horizontal,
                vertical,
                diagonal_bottom_left,
                diagonal_bottom_right,
            ],
        }
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn state(&self) -> &State {
        &self.state
    }

    pub fn column_is_full(&self, column: &Column) -> bool {
        for row in 0..BOARD_HEIGHT {
            let index = row * BOARD_WIDTH + column.0;
            if let Tile::Empty = self.board[index] {
                return false;
            }
        }
        true
    }

    pub fn handle_action(&mut self, action: &Action) {
        match action {
            Action::PlaceTile(column) => match self.state {
                State::Turn(player) => {
                    self.place_tile(column, player);
                    self.update_state(player);
                }
                _ => return,
            },
            Action::Restart(player) => *self = Game::new(player),
        }
    }

    fn place_tile(&mut self, column: &Column, player: Player) {
        for row in 0..BOARD_HEIGHT {
            let i = row * BOARD_WIDTH + column.0;
            if let Tile::Empty = self.board[i] {
                self.board[i] = Tile::Full(player);
                return;
            }
        }
    }

    fn update_state(&mut self, player: Player) {
        if self.check_win(player) {
            self.state = State::Win(player);
            return;
        }

        if self.check_tie() {
            self.state = State::Tie;
            return;
        }

        self.state = match player {
            Player::Player1 => State::Turn(Player::Player2),
            Player::Player2 => State::Turn(Player::Player1),
        };
    }

    fn check_win(&self, player: Player) -> bool {
        for orientation in &self.win_orientations {
            if self.check_win_orientation(orientation, player) {
                return true;
            }
        }
        false
    }

    fn check_win_orientation(&self, o: &WinOrientation, player: Player) -> bool {
        for row in o.row_range.clone() {
            for col in o.col_range.clone() {
                let i = row * BOARD_WIDTH + col;
                if self.board[i] != Tile::Full(player) {
                    continue;
                }

                if self.check_win_at_index(i, &o.offsets, player) {
                    return true;
                }
            }
        }
        false
    }

    fn check_win_at_index(&self, index: usize, win_offsets: &[usize], player: Player) -> bool {
        win_offsets
            .iter()
            .filter(|i| self.board[index + *i] == Tile::Full(player))
            .count()
            == 4
    }

    fn check_tie(&self) -> bool {
        for col in 0..BOARD_WIDTH {
            if !self.column_is_full(&Column::new(col)) {
                return false;
            }
        }
        true
    }
}

impl Player {
    pub fn random(rng: &mut impl Randomizer) -> Self {
        match rng.random_value(0, 1) {
            0 => Self::Player1,
            1 => Self::Player2,
            _ => unreachable!(),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::Player1 => "Player 1",
            Self::Player2 => "Player 2",
        }
    }
}


impl Column {
    pub fn new(i: usize) -> Self {
        assert!(i < BOARD_WIDTH);
        Self(i)
    }
}

impl From<usize> for Column {
    fn from(i: usize) -> Self {
        Self::new(i)
    }
}
