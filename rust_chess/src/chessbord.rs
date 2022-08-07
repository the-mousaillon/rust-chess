use std::collections::{HashMap, HashSet};

use crate::piece::{Color, CanPromoteTo, AttackVector};

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
    pub threatened: bool,
    pub controll: Option<String>
}

// #[derive(Default, Debug, Serialize, Deserialize, Clone)]
// pub struct WebappRepr {
//     pub board: Vec<Vec<CellRepr>>
// }

pub type WebappRepr = Vec<Vec<CellRepr>>;

// impl WebappRepr {
//     pub fn new() -> Self {
//         Self {
//             board: vec![vec![CellRepr::default(); 8] ;8]
//         }
//     }
// }


pub fn empty_board() -> Vec<Vec<Piece>> {
    (0..8).map(|_|
        (0..8).map(|_| Piece::Empty).collect()
    ).collect()
}

pub fn initial_board() -> Vec<Vec<Piece>> {
    let mut board = empty_board();
    // Black faction
    // Major pieces
    board[0][0] = Piece::Rook(Rook::new((0, 0), Color::Black, 1));
    board[0][1] = Piece::Knight(Knight::new((0, 1), Color::Black, 2));
    board[0][2] = Piece::Bishop(Bishop::new((0, 2), Color::Black, 3));
    board[0][3] = Piece::King(King::new((0, 3), Color::Black, 4));
    board[0][4] = Piece::Queen(Queen::new((0, 4), Color::Black, 5));
    board[0][5] = Piece::Bishop(Bishop::new((0, 5), Color::Black, 6));
    board[0][6] = Piece::Knight(Knight::new((0, 6), Color::Black, 7));
    board[0][7] = Piece::Rook(Rook::new((0, 7), Color::Black, 8));
    // Pawns
    for i in 0..8 {
        board[1][i] = Piece::Pawn(Pawn::new((1, i as i8), Color::Black, i * 30));
    }

    // White faction
    // Major pieces
    board[7][0] = Piece::Rook(Rook::new((7, 0), Color::White, 9));
    board[7][1] = Piece::Knight(Knight::new((7, 1), Color::White, 10));
    board[7][2] = Piece::Bishop(Bishop::new((7, 2), Color::White, 11));
    board[7][3] = Piece::Queen(Queen::new((7, 3), Color::White, 12));
    board[7][4] = Piece::King(King::new((7, 4), Color::White, 13));
    board[7][5] = Piece::Bishop(Bishop::new((7, 5), Color::White, 14));
    board[7][6] = Piece::Knight(Knight::new((7, 6), Color::White, 15));
    board[7][7] = Piece::Rook(Rook::new((7, 7), Color::White, 16));
    //Pawns
    for i in 0..8 {
        board[6][i] = Piece::Pawn(Pawn::new((6, i as i8), Color::White, i * 40));
    }

    board
}

pub fn apply_markers(board: &mut WebappRepr, moves: &Vec<Move>) {
    for m in moves {
        match m {
            Move::Move(f, t) => {
                board[t.0 as usize][t.1 as usize].piece.idx = Some(6);
            }
            Move::Take(f, t) => {
                board[t.0 as usize][t.1 as usize].threatened = true;
            },
            Move::EnPassant(from, to) => {
                let pos = (from.0, to.1);
                board[to.0 as usize][to.1 as usize].piece.idx = Some(6);
                board[pos.0 as usize][pos.1 as usize].threatened = true;
            }
            Move::KingsideCastle(c) => {
                let to = m.to().unwrap();
                board[to.0 as usize][to.1 as usize].piece.idx = Some(6);
            }
            Move::QueensideCastle(c) => {
                let to = m.to().unwrap();
                board[to.0 as usize][to.1 as usize].piece.idx = Some(6);
            }
            Move::Invalid => {

            },
            _ => todo!()
        }
    }
}

#[derive(Clone, Debug)]
pub struct Faction {
    pub pieces: HashMap<usize, Piece>,
    // Piece lists
    pub black_pieces: HashMap<usize, Position>,
    pub black_pieces_by_type: HashMap<PieceType, Position>,
    pub white_pieces: HashMap<usize, Position>,
    pub white_pieces_by_type: HashMap<PieceType, Position>,
    // Controled squares for each factions
    pub black_controlled: HashSet<Position>,
    pub white_controlled: HashSet<Position>
}

impl Faction {
    pub fn new_empty() -> Self {
        Self { 
            pieces: HashMap::new(),
            black_pieces: HashMap::new(),
            black_pieces_by_type: HashMap::new(),
            white_pieces: HashMap::new(),
            white_pieces_by_type: HashMap::new(),
            black_controlled: HashSet::new(),
            white_controlled: HashSet::new()
        }
    }

    pub fn upsert(&mut self, piece: Piece) {
        match piece.color() {
            Some(Color::White) => {
                let piece_pos = piece.position().unwrap();
                let piece_type = piece.get_type().unwrap();
                let piece_id = piece.get_id().unwrap();
                self.white_pieces.remove(&piece_id);
                self.white_pieces.insert(piece_id, piece_pos.clone());
                self.white_pieces_by_type.remove(&piece_type);
                self.white_pieces_by_type.insert(piece_type, piece.position().unwrap());
            },
            Some(Color::Black) => {
                let piece_pos = piece.position().unwrap();
                let piece_type = piece.get_type().unwrap();
                let piece_id = piece.get_id().unwrap();
                self.black_pieces.remove(&piece_id);
                self.black_pieces.insert(piece_id, piece_pos.clone());
                self.black_pieces_by_type.remove(&piece_type);
                self.black_pieces_by_type.insert(piece_type, piece.position().unwrap());
            },
            _ => {}
        }
    }

    pub fn delete(&mut self, piece: &Piece) {
        match piece.color() {
            Some(Color::White) => {
                let piece_pos = piece.position().unwrap();
                let piece_type = piece.get_type().unwrap();
                let piece_id = piece.get_id().unwrap();
                self.white_pieces.remove(&piece_id);
                self.white_pieces_by_type.remove(&piece_type);
            },
            Some(Color::Black) => {
                let piece_pos = piece.position().unwrap();
                let piece_type = piece.get_type().unwrap();
                let piece_id = piece.get_id().unwrap();
                self.black_pieces.remove(&piece_id);
                self.black_pieces_by_type.remove(&piece_type);
            },
            _ => {}
        }
    }

    pub fn is_controlled(&self, pos: &Position, color: &Color) -> bool {
        match color {
            Color::Black => self.black_controlled.contains(pos),
            Color::White => self.white_controlled.contains(pos)
        }
    }

    pub fn clear(&mut self) {
        self.black_pieces.clear();
        self.white_pieces.clear();
        self.black_pieces_by_type.clear();
        self.white_pieces_by_type.clear();
    }
}


#[derive(Debug, Clone)]
pub struct ChessBoard {
    pub board: Vec<Vec<Piece>>,
    pub faction: Faction,
    pub headstart: Option<Position>
}

impl ChessBoard {
    pub fn new_default() -> Self {
        let board = initial_board();
        let mut board = Self {
            board: board,
            faction: Faction::new_empty(),
            headstart: None
        };
        board.collect_factions();
        board
    }

    pub fn new_empty() -> Self {
        Self {
            board: empty_board(),
            faction: Faction::new_empty(),
            headstart: None
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

    pub fn locate_king(&self, faction: &Color) -> Position {
        match faction {
            Color::Black => match self.faction.black_pieces_by_type.get(&PieceType::King).unwrap() {
                pos => match self.board[pos.0 as usize][pos.1 as usize] {
                   Piece::King(ref k) => k.position(),
                   _ => {
                        self.pprint();
                        panic!("There should always be king on the board")
                   }
                },
                _ => panic!("There should always be king on the board")
            },
            Color::White => match self.faction.white_pieces_by_type.get(&PieceType::King).unwrap() {
                pos => match self.board[pos.0 as usize][pos.1 as usize] {
                    Piece::King(ref k) => k.position(),
                    _ => {
                        self.pprint();
                        panic!("There should always be king on the board")
                    }
                 },
                 _ => {
                    self.pprint();
                    panic!("There should always be king on the board")
                 }
            }
        }
    }

    pub fn locate_king_mut(&mut self, faction: &Color) -> &mut King {
        match faction {
            Color::Black => match self.faction.black_pieces_by_type.get(&PieceType::King).unwrap() {
                pos => match self.board[pos.0 as usize][pos.1 as usize] {
                   Piece::King(ref mut k) => k,
                   _ => panic!("There should always be king on the board")
                },
                _ => panic!("There should always be king on the board")
            },
            Color::White => match self.faction.white_pieces_by_type.get(&PieceType::King).unwrap() {
                pos => match self.board[pos.0 as usize][pos.1 as usize] {
                    Piece::King(ref mut k) => k,
                    _ => panic!("There should always be king on the board")
                 },
                 _ => panic!("There should always be king on the board")
            }
        }
    }

    pub fn get_board_as_key(&self) -> String {
        let mut repr = String::new();
        repr.reserve(64);
        for i in 0..8 {
            for j in 0..8 {
                repr.push_str(self.board[i][j].emoji());
            }
        }
        repr
    }
    // !! The bord mutations have to be handled outside this function
    pub fn gen_all_moves(&self, player: &Color, check: bool, attack_vectors: &Vec<HashSet<Position>>) -> Vec<Move> {
        match player {
            Color::Black => {
                self.faction.black_pieces
                    .values()
                    .flat_map(|p| {
                        self.board[p.0 as usize][p.1 as usize].gen_moves_check(self, check, attack_vectors)
                    })
                    .filter(|m| m.is_valid(self))
                    .collect()
            },
            Color::White => {
                self.faction.white_pieces
                    .values()
                    .flat_map(|p| {
                        self.board[p.0 as usize][p.1 as usize].gen_moves_check(self, check, attack_vectors)
                    })
                    .filter(|m| m.is_valid(self))
                    .collect()
            }
        }
    }

    pub fn reset_pin_vectors(&mut self, faction: &Color) {
        match faction {
            Color::Black => {
                for pos in self.faction.black_pieces.values() {
                    self.board[pos.0 as usize][pos.1 as usize].set_attack_vector(None);
                }
            },
            Color::White => {
                for pos in self.faction.white_pieces.values() { 
                    self.board[pos.0 as usize][pos.1 as usize].set_attack_vector(None);
                }
            }
        }
    }

    pub fn update_controlled_squares(&mut self, faction: &Color) {
        match faction {
            Color::Black => {
                let mut black_controlled = HashSet::new();
                for pos in self.faction.black_pieces.values() {
                    let piece = &self.board[pos.0 as usize][pos.1 as usize];
                    black_controlled.extend(piece.get_controlled_squares(self));
                }
                self.faction.black_controlled = black_controlled;
            },
            Color::White => {
                let mut white_controlled = HashSet::new();
                for pos in self.faction.white_pieces.values() {
                    let piece = &self.board[pos.0 as usize][pos.1 as usize];
                    white_controlled.extend(piece.get_controlled_squares(self));
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

    // returns the promotion position if the move leads to a promotion
    pub fn play_once(&mut self, m: Move) -> Option<Position> {
        let mut headstart = None;
        let mut promotion = None;
        match m {
            Move::Take(from, to) => {
                let mut p = self.board[from.0 as usize][from.1 as usize].clone();
                let piece_to_delete = &self.board[to.0 as usize][to.1 as usize];
                p.set_position(to.clone());
                self.faction.upsert(p.clone());
                self.faction.delete(piece_to_delete);
                self.board[from.0 as usize][from.1 as usize] = Piece::Empty;
                let piece_color = p.color().unwrap();
                let piece_type = p.get_type().unwrap();
                self.board[to.0 as usize][to.1 as usize] = p;
                match (piece_type, piece_color, to) {
                    (PieceType::Pawn, Color::Black, to @ (7, _)) => promotion = Some(to),
                    (PieceType::Pawn, Color::White, to @ (0, _)) => promotion = Some(to),
                    _ => {}
                }
            },
            Move::Move(from, to) => {
                let mut p = self.board[from.0 as usize][from.1 as usize].clone();
                // If we have a pawn headstart
                match p {
                    Piece::Pawn(ref mut p)if !p.has_moved && (p.position.0 - to.0).abs() == 2 => {
                        p.headstart = true;
                        headstart = Some(to)
                    },
                    _ => {}
                }
                p.set_position(to.clone());
                self.faction.upsert(p.clone());
                self.board[from.0 as usize][from.1 as usize] = Piece::Empty;
                let piece_color = p.color().unwrap();
                let piece_type = p.get_type().unwrap();
                self.board[to.0 as usize][to.1 as usize] = p;
                match (piece_type, piece_color, to) {
                    (PieceType::Pawn, Color::Black, to @ (7, _)) => promotion = Some(to),
                    (PieceType::Pawn, Color::White, to @ (0, _)) => promotion = Some(to),
                    _ => {}
                }
            },
            Move::EnPassant(from, to) => {
                let mut p = self.board[from.0 as usize][from.1 as usize].clone();
                let piece_to_delete = &self.board[from.0 as usize][to.1 as usize];
                p.set_position(to.clone());
                self.faction.upsert(p.clone());
                self.faction.delete(piece_to_delete);
                self.board[from.0 as usize][from.1 as usize] = Piece::Empty;
                self.board[from.0 as usize][to.1 as usize] = Piece::Empty;
                let piece_color = p.color().unwrap();
                self.board[to.0 as usize][to.1 as usize] = p;
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
            },
            Move::Promote(from, to_type) => {
                let piece = &self.board[from.0 as usize][from.1 as usize];
                let piece_color = piece.color().unwrap();
                let piece_id = piece.get_id().unwrap();
                let new_piece = Piece::new(from, piece_color.clone(), to_type.into(), piece_id);
                self.faction.upsert(new_piece.clone());
                self.board[from.0 as usize][from.1 as usize] = new_piece;
            }
            _ => {}
            // We update the controled squares for each faction
        }
        self.update_headstart(headstart);
        promotion
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

    // 1. white headstart -> update(new_p) // Black headstart -> update(new_p) // black otherwise -> update(None)
    pub fn update_headstart(&mut self, new_p: Option<Position>) {
        if let Some(p) = self.headstart {
            match self.board[p.0 as usize][p.1 as usize] {
                Piece::Pawn(ref mut p) => {
                    p.headstart = false;
                },
                _ => {}
            }
        }
        self.headstart = new_p;
    }

    pub fn to_webapp(&self) -> WebappRepr {
        let mut repr = vec![vec![CellRepr::default(); 8]; 8];
        for i in 0..8 {
            for j in 0..8 {
                let mut cell_repr = self.board[i][j].webapp_repr();
                let white_controll = self.faction.is_controlled(&(i as i8, j as i8), &Color::White);
                let black_controll = self.faction.is_controlled(&(i as i8, j as i8), &Color::Black);

                match (white_controll, black_controll) {
                    (true, true) => cell_repr.controll = Some("BOTH".into()),
                    (false, true) => cell_repr.controll = Some("WHITE".into()),
                    (true, false) => cell_repr.controll = Some("BLACK".into()),
                    _ => {}
                }
                repr[i][j] = cell_repr;
            }
        }
        repr
    }
}

