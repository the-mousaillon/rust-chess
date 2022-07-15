use std::collections::HashMap;

use crate::piece::Color;

use super::piece::{
    Piece,
    PieceCommon,
    Position,
    Rook,
    King,
    Knight,
    Pawn,
    Bishop,
    Queen
};

pub fn empty_board() -> Vec<Vec<Piece>> {
    (0..8).map(|_|
        (0..8).map(|_| Piece::Empty).collect()
    ).collect()
}

pub fn initial_board() -> Vec<Vec<Piece>> {
    let mut board = empty_board();
    // Black faction
    // Major pieces
    board[0][0] = Piece::Rook(Rook::new((0, 0), Color::Black));
    board[0][1] = Piece::Knight(Knight::new((0, 1), Color::Black));
    board[0][2] = Piece::Bishop(Bishop::new((0, 2), Color::Black));
    board[0][3] = Piece::King(King::new((0, 3), Color::Black));
    board[0][4] = Piece::Queen(Queen::new((0, 4), Color::Black));
    board[0][5] = Piece::Bishop(Bishop::new((0, 5), Color::Black));
    board[0][6] = Piece::King(King::new((0, 6), Color::Black));
    board[0][7] = Piece::Rook(Rook::new((0, 7), Color::Black));
    // Pawns
    for i in 0..8 {
        board[1][i] = Piece::Pawn(Pawn::new((1, i as u8), Color::Black));
    }

    // White faction
    // Major pieces
    board[7][0] = Piece::Rook(Rook::new((7, 0), Color::White));
    board[7][1] = Piece::King(King::new((7, 1), Color::White));
    board[7][2] = Piece::Bishop(Bishop::new((7, 3), Color::White));
    board[7][3] = Piece::Queen(Queen::new((7, 3), Color::White));
    board[7][4] = Piece::King(King::new((7, 4), Color::White));
    board[7][5] = Piece::Bishop(Bishop::new((7, 5), Color::White));
    board[7][6] = Piece::Knight(Knight::new((7, 6), Color::White));
    board[7][7] = Piece::Rook(Rook::new((7, 7), Color::White));
    //Pawns
    for i in 0..8 {
        board[6][i] = Piece::Pawn(Pawn::new((1, i as u8), Color::White));
    }

    board
}


#[derive(Debug, Clone)]
pub struct ChessBoard {
    pub board: Vec<Vec<Piece>>,
    pub white_faction: HashMap<Position, Piece>,
    pub black_faction: HashMap<Position, Piece>,
}

impl ChessBoard {
    pub fn new_default() -> Self {
        let board = initial_board();
        let mut board = Self {
            board: board,
            white_faction: HashMap::new(),
            black_faction: HashMap::new(),
        };
        board.collect_factions();
        board
    }

    pub fn collect_factions(&mut self) {
        self.black_faction.clear();
        self.white_faction.clear();
        for i in 0..8 {
            for j in 0..8 {
                match self.board[i][j].color() {
                    Some(Color::White) => {
                        self.white_faction.insert((i as u8, j as u8), self.board[i][j].clone());
                    },
                    Some(Color::Black) => {
                        self.black_faction.insert((i as u8, j as u8), self.board[i][j].clone());
                    },
                    _ => {}
                }
            }
        }
    }

    pub fn pprint(&self) {
        let sep_row = format!("|{}|", vec!("_"; 8).join("|"));
        for i in 0..8 {
            let mut row = vec!();
            for j in 0..8 {
                row.push(self.board[i][j].emoji());
            }
            let row = row.join("|");
            let row = format!("|{}|", row);
            println!("{}", row);
            println!("{}", sep_row);
        }
    }
}

#[test]
fn intial_board() {
    let board = ChessBoard::new_default();
    board.pprint();
}