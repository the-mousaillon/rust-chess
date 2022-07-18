use std::collections::{HashMap, HashSet};

use crate::piece::Color;

use super::piece::{
    Piece,
    PieceType,
    PieceCommon,
    Position,
    Rook,
    King,
    Knight,
    Pawn,
    Bishop,
    Queen,
    Move,
};

use serde::{Serialize, Deserialize};


#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct PieceRepr {
    pub idx: Option<usize>,
    pub color: Option<String>,
    pub name: Option<String>
}


#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct CellRepr {
    pub piece: PieceRepr,
    pub threatened: bool
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct WebappRepr {
    pub board: Vec<Vec<CellRepr>>
}

impl WebappRepr {
    pub fn new() -> Self {
        Self {
            board: vec![vec![CellRepr::default(); 8] ;8]
        }
    }
}


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
    board[0][6] = Piece::Knight(Knight::new((0, 6), Color::Black));
    board[0][7] = Piece::Rook(Rook::new((0, 7), Color::Black));
    // Pawns
    for i in 0..8 {
        board[1][i] = Piece::Pawn(Pawn::new((1, i as i8), Color::Black));
    }

    // White faction
    // Major pieces
    board[7][0] = Piece::Rook(Rook::new((7, 0), Color::White));
    board[7][1] = Piece::Knight(Knight::new((7, 1), Color::White));
    board[7][2] = Piece::Bishop(Bishop::new((7, 2), Color::White));
    board[7][3] = Piece::Queen(Queen::new((7, 3), Color::White));
    board[7][4] = Piece::King(King::new((7, 4), Color::White));
    board[7][5] = Piece::Bishop(Bishop::new((7, 5), Color::White));
    board[7][6] = Piece::Knight(Knight::new((7, 6), Color::White));
    board[7][7] = Piece::Rook(Rook::new((7, 7), Color::White));
    //Pawns
    for i in 0..8 {
        board[6][i] = Piece::Pawn(Pawn::new((6, i as i8), Color::White));
    }

    board
}

pub fn apply_markers(board: &mut WebappRepr, moves: &Vec<Move>) {
    for m in moves {
        match m {
            Move::Move(f, t) => {
                board.board[t.0 as usize][t.1 as usize].piece.idx = Some(6);
            }
            Move::Take(f, t) => {
                board.board[t.0 as usize][t.1 as usize].threatened = true;
            },
            Move::EnPassant(from, to) => {
                let pos = (from.0, to.1);
                board.board[to.0 as usize][to.1 as usize].piece.idx = Some(6);
                board.board[pos.0 as usize][pos.1 as usize].threatened = true;
            }
            Move::KingsideCastle(c) => {
                let to = m.to().unwrap();
                board.board[to.0 as usize][to.1 as usize].piece.idx = Some(6);
            }
            Move::QueensideCastle(c) => {
                let to = m.to().unwrap();
                board.board[to.0 as usize][to.1 as usize].piece.idx = Some(6);
            }
            Move::Invalid => {

            },
            _ => todo!()
        }
    }
}

#[derive(Clone, Debug)]
pub struct Faction {
    // Piece lists
    pub black_pieces_by_pos: HashMap<Position, Piece>,
    pub black_pieces_by_type: HashMap<PieceType, Piece>,
    pub white_pieces_by_pos: HashMap<Position, Piece>,
    pub white_pieces_by_type: HashMap<PieceType, Piece>,
    // Controled squares for each factions
    pub black_controlled: HashSet<Position>,
    pub white_controlled: HashSet<Position>
}

impl Faction {
    pub fn new_empty() -> Self {
        Self { 
            black_pieces_by_pos: HashMap::new(),
            black_pieces_by_type: HashMap::new(),
            white_pieces_by_pos: HashMap::new(),
            white_pieces_by_type: HashMap::new(),
            black_controlled: HashSet::new(),
            white_controlled: HashSet::new()
        }
    }

    pub fn upsert(&mut self, piece: Piece) {
        match piece.color() {
            Some(Color::White) => {
                let piece_pos = piece.position().unwrap();
                self.white_pieces_by_pos.remove(&piece_pos);
                self.white_pieces_by_pos.insert(piece_pos, piece.clone());
                let piece_type = piece.get_type().unwrap();
                self.white_pieces_by_type.remove(&piece_type);
                self.white_pieces_by_type.insert(piece_type, piece);
            },
            Some(Color::Black) => {
                let piece_pos = piece.position().unwrap();
                self.black_pieces_by_pos.remove(&piece_pos);
                self.black_pieces_by_pos.insert(piece_pos, piece.clone());
                let piece_type = piece.get_type().unwrap();
                self.black_pieces_by_type.remove(&piece_type);
                self.black_pieces_by_type.insert(piece_type, piece);
            },
            _ => {}
        }
    }

    pub fn clear(&mut self) {
        self.black_pieces_by_pos.clear();
        self.black_pieces_by_type.clear();
        self.white_pieces_by_pos.clear();
        self.white_pieces_by_type.clear();
    }
}


#[derive(Debug, Clone)]
pub struct ChessBoard {
    pub board: Vec<Vec<Piece>>,
    pub faction: Faction,
}

impl ChessBoard {
    pub fn new_default() -> Self {
        let board = initial_board();
        let mut board = Self {
            board: board,
            faction: Faction::new_empty(),
        };
        board.collect_factions();
        board
    }

    pub fn new_empty() -> Self {
        Self {
            board: empty_board(),
            faction: Faction::new_empty()
        }
    }

    pub fn collect_factions(&mut self) {
        self.faction.clear();
        for i in 0..8 {
            for j in 0..8 {
                match self.board[i][j].color() {
                    Some(_) => {
                        self.faction.upsert(self.board[i][j].clone());
                    },
                    _ =>{}
                }
            }
        }
    }

    pub fn locate_king(&self, faction: Color) -> King {
        match faction {
            Color::Black => match self.faction.black_pieces_by_type.get(&PieceType::King).unwrap() {
                Piece::King(k) => k.clone(),
                _ => panic!("There should always be king on the board")
            },
            Color::White => match self.faction.white_pieces_by_type.get(&PieceType::King).unwrap() {
                Piece::King(k) => k.clone(),
                _ => panic!("There should always be king on the board")
            }
        }
    }

    pub fn update_controlled_squares(&mut self, faction: Color) {
        match faction {
            Color::Black => {
                let mut black_controlled = HashSet::new();
                for (_, bp) in self.faction.black_pieces_by_pos.iter() {
                    black_controlled.extend(bp.get_controlled_squares(self));
                }
                self.faction.black_controlled = black_controlled;
            },
            Color::White => {
                let mut white_controlled = HashSet::new();
                for (_, wp) in self.faction.white_pieces_by_pos.iter() {
                    white_controlled.extend(wp.get_controlled_squares(self));
                }
                self.faction.white_controlled = white_controlled;
            }
        }
    }

    fn castle(&mut self, king_from: Position, king_to: Position, rook_from: Position, rook_to: Position){
        let mut king = self.board[king_from.0 as usize][king_from.1 as usize].clone();
        let mut rook = self.board[rook_from.0 as usize][rook_from.1 as usize].clone();
        king.set_position(king_to);
        rook.set_position(rook_to);
        self.faction.upsert(king.clone());
        self.faction.upsert(rook.clone());
        self.board[king_from.0 as usize][king_from.1 as usize] = Piece::Empty;
        self.board[rook_from.0 as usize][rook_from.1 as usize] = Piece::Empty;
        self.board[king_to.0 as usize][king_to.1 as usize] = king;
        self.board[rook_to.0 as usize][rook_to.1 as usize] = rook;
    }

    pub fn play(&mut self, moves: Vec<Move>) {
        for m in moves {
            match m {
                Move::Take(from, to) => {
                    let mut p = self.board[from.0 as usize][from.1 as usize].clone();
                    p.set_position(to.clone());
                    self.faction.upsert(p.clone());
                    self.board[from.0 as usize][from.1 as usize] = Piece::Empty;
                    let piece_color = p.color().unwrap();
                    self.board[to.0 as usize][to.1 as usize] = p;
                    self.update_controlled_squares(piece_color);
                },
                Move::Move(from, to) => {
                    let mut p = self.board[from.0 as usize][from.1 as usize].clone();
                    p.set_position(to.clone());
                    self.faction.upsert(p.clone());
                    self.board[from.0 as usize][from.1 as usize] = Piece::Empty;
                    let piece_color = p.color().unwrap();
                    self.board[to.0 as usize][to.1 as usize] = p;
                    self.update_controlled_squares(piece_color);
                },
                Move::EnPassant(from, to) => {
                    let mut p = self.board[from.0 as usize][from.1 as usize].clone();
                    p.set_position(to.clone());
                    self.faction.upsert(p.clone());
                    self.board[from.0 as usize][from.1 as usize] = Piece::Empty;
                    self.board[from.0 as usize][to.1 as usize] = Piece::Empty;
                    let piece_color = p.color().unwrap();
                    self.board[to.0 as usize][to.1 as usize] = p;
                    self.update_controlled_squares(piece_color);
                },
                Move::KingsideCastle(faction) => {
                    match faction {
                        Color::White => {
                            self.castle((7, 4), (7, 6), (7, 7), (7, 5))
                        }
                        Color::Black => {
                            self.castle((0, 3), (0, 1), (0, 0), (0, 2))
                        }
                    }
                    self.update_controlled_squares(faction);
                },
                Move::QueensideCastle(faction) => {
                    match faction {
                        Color::White => {
                            self.castle((7, 4), (7, 2), (7, 0), (7, 3))
                        }
                        Color::Black => {
                            self.castle((0, 3), (0, 5), (0, 7), (0, 4))
                        }
                    }
                    self.update_controlled_squares(faction);
                },
                _ => {}
                // We update the controled squares for each faction
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

    pub fn to_webapp(&self) -> WebappRepr {
        let mut repr = WebappRepr::new();
        for i in 0..8 {
            for j in 0..8 {
                repr.board[i][j] = self.board[i][j].webapp_repr();
            }
        }
        repr
    }
}
