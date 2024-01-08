use raylib::prelude::*;

fn main() -> Result<()> {
    let view = BoardView {
        tile_size: 128.0,
        tile_padding: 8.0,
        board_padding: 32.0,
    };
    let (width, height) = view.window_size();

    let (mut rl, thread) = raylib::init()
        .size(width as i32, height as i32)
        .title("Connect 4")
        .build();

    let pallete = Pallete {
        background_color: Color::from_hex("191923").unwrap(),
        board_color: Color::from_hex("FBFEF9").unwrap(),
        player_1_color: Color::from_hex("0E79B2").unwrap(),
        player_2_color: Color::from_hex("F39237").unwrap(),
    };

    let mut board = Board::empty();
    board.place_piece(Some(0), Piece::Player1)?;
    board.place_piece(Some(1), Piece::Player2)?;

    let mut state = State::Player1Turn;

    while !rl.window_should_close() {

        /* Update */
        match state {
            State::Player1Win => {
                //d.draw_text("Player 1 wins!", 12, 12, 12, pallete.player_1_color);
                break;
            }
            State::Player2Win => {
                //d.draw_text("Player 2 wins!", 12, 12, 12, pallete.player_2_color);
                break;
            }
            _ => {}
        }

        // input
        // Get mouse position and translate it to board coordinates
        let mouse_pos = rl.get_mouse_position();
        //println!("{:?}", mouse_pos);
        let column = get_board_column(&view, mouse_pos);

        // Place piece in column
        match state {
            State::Player1Turn => {
                board.place_piece(column , Piece::Player1)?;
                state = State::Player2Turn;
            }
            State::Player2Turn => {
                board.place_piece(column, Piece::Player2)?;
                state = State::Player1Turn;
            }
            _ => {}
        }

        // Check for win
        let win = check_win(&board, &state);
        if let Some(win) = win {
            match win {
                State::Player1Win | State::Player2Win => {
                    state = win;
                }
                _ => {}
            }
        }

        /* Draw */
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(pallete.background_color);
        draw_board(&mut d, &board, &pallete, &view);

        //d.draw_text("Player 1", 12, 12, 12, pallete.player_1_color);
        //d.draw_text("Player 2", 12, 24, 12, pallete.player_2_color);
    }

    Result::Ok(())
}

struct BoardView {
    tile_size: f32,
    tile_padding: f32,
    board_padding: f32,
}

impl BoardView {
    fn window_size(&self) -> (f32, f32) {
        let width = self.tile_size * 7.0 + self.board_padding * 2.0;
        let height = self.tile_size * 6.0 + self.board_padding * 2.0;
        (width, height)
    }

    fn board_size(&self) -> (f32, f32) {
        let width = self.tile_size * 7.0;
        let height = self.tile_size * 6.0;
        (width, height)
    }

    fn board_position(&self) -> (f32, f32) {
        let x = self.board_padding;
        let y = self.board_padding;
        (x, y)
    }

    fn piece_position(&self, row: usize, column: usize) -> (f32, f32) {
        let x = self.board_padding + self.tile_size * column as f32 + self.tile_size / 2.0;
        let y = self.board_padding + self.tile_size * (5 - row) as f32 + self.tile_size / 2.0;
        (x, y)
    }

    fn piece_radius(&self) -> f32 {
        self.tile_size / 2.0 - self.tile_padding
    }
}

struct Pallete {
    background_color: Color,
    board_color: Color,
    player_1_color: Color,
    player_2_color: Color,
}

#[derive(Copy, Clone, PartialEq)]
enum Piece {
    Empty,
    Player1,
    Player2,
}

struct Board {
    pieces: [[Piece; 7]; 6],
}

impl Board {
    fn empty() -> Self {
        Self {
            pieces: [[Piece::Empty; 7]; 6],
        }
    }

    fn place_piece(&mut self, column: Option<usize>, piece: Piece) -> Result<()> {
        match column {
            Some(column) => {
                if self.pieces[5][column] != Piece::Empty {
                    return Result::Err(InvalidMove);
                }
            }
            None => {
                let mut column = 0;
                while self.pieces[5][column] != Piece::Empty {
                    column += 1;
                    if column >= 7 {
                        return Result::Err(InvalidMove);
                    }
                }
            }
        }
        Result::Ok(())
    }
}

#[derive(Debug)]
struct InvalidMove;

type Result<T> = std::result::Result<T, InvalidMove>;

impl std::fmt::Display for InvalidMove {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Invalid move")
    }
}

fn draw_board(d: &mut RaylibDrawHandle, board: &Board, pallete: &Pallete, view: &BoardView) {
    let (x, y) = view.board_position();
    let (width, height) = view.board_size();
    d.draw_rectangle(
        x as i32,
        y as i32,
        width as i32,
        height as i32,
        pallete.board_color,
    );

    for row in 0..6 {
        for column in 0..7 {
            let (x, y) = view.piece_position(row, column);
            let radius = view.piece_radius();
            let color = match board.pieces[row][column] {
                Piece::Empty => pallete.background_color,
                Piece::Player1 => pallete.player_1_color,
                Piece::Player2 => pallete.player_2_color,
            };

            d.draw_circle(x as i32, y as i32, radius as f32, color);
        }
    }
}

enum State {
    Player1Turn,
    Player2Turn,
    Player1Win,
    Player2Win,
}

fn get_board_column(view: &BoardView, mouse_pos: Vector2) -> Option<usize> {
    let (x, _) = view.board_position();
    let mouse_x = mouse_pos.x - x;
    let column = (mouse_x / view.tile_size) as usize;
    if column < 7 {
        Some(column)
    } else {
        None
    }
}

fn check_win(board: &Board, state: &State) -> Option<State> {
    None
}
