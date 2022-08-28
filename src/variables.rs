pub mod pieces;
use crate::variables::pieces::Piece;


enum State { Initializing = -1, Playing, Won, Lost, Draw}

struct Board{
    state: State,
    choices: [[bool; 8]; 8],
    pieces: Vec<Piece>,
    pending: (u8, u8),
    white_king: Piece,
    black_king: Piece,
    transposition: std::collections::HashMap<(State, i32), f32>,
    state_cache: std::collections::HashMap<State, State>,
    cached_board: [[Piece; 8]; 8]
}