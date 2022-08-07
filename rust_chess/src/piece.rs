use std::collections::{HashMap, HashSet};

use serde::{Serialize, Deserialize};

use crate::chessbord::Faction;

use super::chessbord::{
    ChessBoard,
    CellRepr,
    PieceRepr
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Color {
    Black,
    White,
}

impl Color {
    pub fn webapp_repr(&self) -> String {
        match self {
            Self::Black => "BLACK".into(),
            Self::White => "WHITE".into()
        }
    }

    pub fn other(&self) -> Color {
        match self {
            Self::Black => Self::White,
            Self::White => Self::Black
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum CanPromoteTo {
    Rook,
    Bishop,
    Knight,
    Queen
}

impl Into<PieceType> for CanPromoteTo {
    fn into(self) -> PieceType {
        match self {
            Self::Rook => PieceType::Rook,
            Self::Bishop => PieceType::Bishop,
            Self::Knight => PieceType::Knight,
            Self::Queen => PieceType::Queen,
        }
    }
}


#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PieceType {
    Pawn,
    Knight,
    Rook,
    Queen,
    King,
    Bishop,
    Empty,
}

#[derive(Clone, Debug)]
pub enum Piece {
    Pawn(Pawn),
    Knight(Knight),
    Rook(Rook),
    Queen(Queen),
    King(King),
    Bishop(Bishop),
    Empty,
}

fn get_piece_color(p: &impl PieceCommon) -> Option<Color> {
    Some(p.color())
}

fn get_piece_position(p: &impl PieceCommon) -> Option<Position> {
    Some(p.position())
}

fn pprint_piece(p: &impl PieceCommon) -> &'static str {
    p.emoji_repr()
}

fn get_piece_type(p: &impl PieceCommon) -> Option<PieceType> {
    Some(p.get_type())
}

fn get_piece_id(p: &impl PieceCommon) -> Option<usize> {
    Some(p.get_id())
}

macro_rules! multi_match {
    ($p: ident, $match: ident, $no_match: ident) => {
        match $p {
            Piece::Pawn(p) => $match(p),
            Piece::Knight(p) => $match(p),
            Piece::Rook(p) => $match(p),
            Piece::Queen(p) => $match(p),
            Piece::King(p) => $match(p),
            Piece::Bishop(p) => $match(p),
            Piece::Empty => $no_match,
        }
    }
}

impl Piece {
    pub fn new(from: Position, color: Color, ptype: PieceType, id: usize) -> Self {
        match ptype {
            PieceType::Pawn => Self::Pawn(Pawn::new(from, color, id)),
            PieceType::Knight => Self::Knight(Knight::new(from, color, id)),
            PieceType::Rook => Self::Rook(Rook::new(from, color, id)),
            PieceType::Queen => Self::Queen(Queen::new(from, color, id)),
            PieceType::King => Self::King(King::new(from, color, id)),
            PieceType::Bishop => Self::Bishop(Bishop::new(from, color, id)),
            PieceType::Empty => Self::Empty,
        }
    }

    pub fn color(&self) -> Option<Color> {
        multi_match!(self, get_piece_color, None)
    }

    pub fn get_type(&self) -> Option<PieceType> {
        multi_match!(self, get_piece_type, None)
    }


    pub fn get_id(&self) -> Option<usize> {
        multi_match!(self, get_piece_id, None)
    }

    pub fn position(&self) -> Option<Position> {
        multi_match!(self, get_piece_position, None)
    }

    pub fn pprint(&self) {
        let emoj = self.emoji();
        print!("{}", emoj);
    }

    pub fn emoji(&self) -> &'static str {
        let empty = " ";
        multi_match!(self, pprint_piece, empty)
    }

    pub fn is_empty(&self) -> bool {
        match self {
            Piece::Empty => true,
            _ => false
        }
    }

    // We lose a bit of efficiency since we generate each moves twice, but it makes code tighter and simpler
    pub fn get_controlled_squares(&self, board: &ChessBoard) -> Vec<Position> {
        let mut controlled = vec!();
        match self.get_type() {
            Some(PieceType::Pawn) => {
                let (x, y) = self.position().unwrap();
                match self.color().unwrap() {
                    Color::Black => {
                        controlled.push((x + 1, y + 1));
                        controlled.push((x + 1, y - 1));
                    },
                    Color::White => {
                        controlled.push((x - 1, y + 1));
                        controlled.push((x - 1, y - 1));
                    }
                }
            },
            _ => {
                for m in self.gen_moves(board) {
                    if let Move::Move(_, to) = m {
                        controlled.push(to)
                    }
                    if let Move::Defend(p) = m {
                        controlled.push(p)
                    }
                }
            }
        }
        controlled
    }

    pub fn gen_moves_check(&self, board: &ChessBoard, check: bool, attack_vectors: &Vec<HashSet<Position>>) -> Vec<Move> {
        if !check {
            return self.gen_moves(board)
        }

        match self {
            Piece::King(k) => k.gen_moves(board),
            Piece::Empty => vec!(),
            other @ _ if attack_vectors.len() == 1 => {
                other.gen_moves(board)
                    .into_iter()
                    .filter(|m| {
                        match m {
                            Move::Move(_, to) => {
                                attack_vectors.iter().find(|hm| hm.contains(to)).is_some()
                            },
                            Move::Take(_, to) => {
                                attack_vectors.iter().find(|hm| hm.contains(to)).is_some()
                            },
                            _ => false
                        }
                    })
                    .collect()
            },
            _ => vec!()
        }
    }

    pub fn gen_moves(&self, board: &ChessBoard) -> Vec<Move> {
        match self {
            Piece::Pawn(p) => p.gen_moves(board),
            Piece::Knight(p) => p.gen_moves(board),
            Piece::Rook(p) => p.gen_moves(board),
            Piece::Queen(p) => p.gen_moves(board),
            Piece::King(p) => p.gen_moves(board),
            Piece::Bishop(p) => p.gen_moves(board),
            Piece::Empty => vec!(),
        }
    }

    pub fn can_attack(&self, direction: &VectorDirection) -> bool {
        match self {
            Piece::Rook(p) => direction.is_line(),
            Piece::Queen(p) => direction.is_line() || direction.is_diagonal(),
            Piece::Bishop(p) => direction.is_diagonal(),
            _ => false
        }
    }

    pub fn set_position(&mut self, pos: Position) {
        match self {
            Piece::Pawn(p) => {
                p.position = pos;
                p.has_moved = true;
            },
            Piece::Knight(p) => p.position = pos,
            Piece::Rook(p) => {
                p.position = pos;
                p.has_moved = true;
            },
            Piece::Queen(p) => p.position = pos,
            Piece::King(p) => {
                p.position = pos;
                p.has_moved = true;
            },
            Piece::Bishop(p) => p.position = pos,
            Piece::Empty => {},
        }
    }

    pub fn set_attack_vector(&mut self, direction: Option<VectorDirection>) {
        match self {
            Piece::Pawn(p) => {
                p.pin_vector = direction;
            },
            Piece::Knight(p) => {
                p.pin_vector = direction;
            },
            Piece::Rook(p) => {
                p.pin_vector = direction;
            },
            Piece::Queen(p) => {
                p.pin_vector = direction;
            },
            Piece::Bishop(p) => {
                p.pin_vector = direction;
            },
            _ => {}
        }
    }

    pub fn webapp_repr(&self) -> CellRepr {
        let color = self.color();
        match self {
            Piece::King(p) => CellRepr {
                piece: PieceRepr {
                    idx: Some(0),
                    color: color.map(|c| c.webapp_repr()),
                    name: Some("KING".into())
                },
                threatened: false,
                controll: None,
            },
            Piece::Queen(p) => CellRepr {
                piece: PieceRepr {
                    idx: Some(1),
                    color: color.map(|c| c.webapp_repr()),
                    name: Some("QUEEN".into())
                },
                threatened: false,
                controll: None,
            },
            Piece::Rook(p) => CellRepr {
                piece: PieceRepr {
                    idx: Some(2),
                    color: color.map(|c| c.webapp_repr()),
                    name: Some("ROOK".into())
                },
                threatened: false,
                controll: None,
            },
            Piece::Bishop(p) => CellRepr {
                piece: PieceRepr {
                    idx: Some(3),
                    color: color.map(|c| c.webapp_repr()),
                    name: Some("BISHOP".into())
                },
                threatened: false,
                controll: None,
            },
            Piece::Knight(p) => CellRepr {
                piece: PieceRepr {
                    idx: Some(4),
                    color: color.map(|c| c.webapp_repr()),
                    name: Some("KNIGHT".into())
                },
                threatened: false,
                controll: None,
            },
            Piece::Pawn(p) => CellRepr {
                piece: PieceRepr {
                    idx: Some(5),
                    color: color.map(|c| c.webapp_repr()),
                    name: Some("PAWN".into())
                },
                threatened: false,
                controll: None,
            },
            Piece::Empty => CellRepr {
                piece: PieceRepr {
                    idx: Some(10),
                    color: color.map(|c| c.webapp_repr()),
                    name: Some("EMPTY".into())
                },
                threatened: false,
                controll: None,
            },
        }
    }
}

pub type Position = (i8, i8);


//// MOVES 
#[derive(Clone, Debug, PartialEq)]
pub enum Move {
    Move(Position, Position),
    Take(Position, Position),
    EnPassant(Position, Position),
    KingsideCastle(Color),
    QueensideCastle(Color),
    Promote(Position, CanPromoteTo),
    Defend(Position),
    Invalid
}

impl Move {
    // This function should be called only for move or take moves. Castling and en passant would make this function pannic
    pub fn new(p: &impl PieceCommon, board: &ChessBoard, to: Position) -> Self {
        if to.0 < 0 || to.0 > 7 || to.1 < 0 || to.1 > 7 {
            return Move::Invalid
        }
        let dst = &board.board[to.0 as usize][to.1 as usize];

        if !dst.is_empty() && (dst.color().unwrap() == p.color()) {
            return Move::Defend(to)
        }
        // Otherwise we either move or take
        match dst {
            Piece::Empty => Self::Move(p.position(), to),
            // We allow to pass through enemy king for controlled squares
            Piece::King(k) if k.color != p.color() => Self::Move(p.position(), to),
            p_ => Move::Take(p.position(), to),
        }
    }

    pub fn new_with_enpassant(p: &impl PieceCommon, board: &ChessBoard, to: Position) -> Self {
        let m = Self::new(p, board, to);
        // If the move is a valid take, we return the move
        match m {
            Move::Take(_, _) => return m,
            _ => {}
        }
        // Otherwise we check for enpassant
        let to_enpassant = (p.position().0, to.1);
        match Self::new(p, board, to_enpassant) {
            // If we have a take
            Move::Take(from, to_take) => {
                if let Piece::Pawn(ref p) = board.board[to_take.0 as usize][to_take.1 as usize] {
                    if p.headstart {
                        Self::EnPassant(from, to)
                    }
                    else {
                        Self::Invalid
                    }
                }
                else {
                    Self::Invalid
                }
            },
            _ => Self::Invalid
        }
    }

    pub fn to(&self) -> Option<(i8, i8)> {
        match self {
            Self::Move(_, to) => Some(to.clone()),
            Self::Take(_, to) => Some(to.clone()),
            Self::EnPassant(_, to) => Some(to.clone()),
            Self::KingsideCastle(c) => match c {
                Color::Black => Some((0, 1)),
                Color::White => Some((7, 6))
            },
            Self::QueensideCastle(c) => match c {
                Color::Black => Some((0, 6)),
                Color::White => Some((7, 1))                
            }
            _ => None
        }
    }

    pub fn is_promotable(&self, board: &ChessBoard) -> bool {
        match self {
            Self::Move(_, to) => {
                let dst = &board.board[to.0 as usize][to.1 as usize];
                match (dst.get_type(), dst.color(), to) {
                    (Some(PieceType::Pawn), Some(Color::Black), (0, _)) => true,
                    (Some(PieceType::Pawn), Some(Color::White), (7, _)) => true,
                    _ => false
                }
            },
            _ => false
        }
    }

    pub fn is_valid(&self, board: &ChessBoard) -> bool {
        match self {
            Self::Invalid => false,
            Self::Defend(_) => false,
            Self::Take(_, to) |  Self::Move(_, to) => {
                board.board[to.0 as usize][to.1 as usize].get_type() != Some(PieceType::King)
            }
            _ => true
        }
    }
}

fn diag_line_generic(p: &impl PieceCommon, board: &ChessBoard, n_travel: i8, pos_trans: impl Fn(i8) -> Position) -> Vec<Move> {
    let mut moves = vec!();
    for i in 1..n_travel {
        let to = pos_trans(i);
        let move_ = Move::new(p, board, to);
        match move_ {
            Move::Move(_, _) =>{
                moves.push(move_);
            },
            Move::Take(_, _) => {
                moves.push(move_);
                break;
            },
            Move::Invalid => {
                break;
            },
            Move::Defend(_) => {
                moves.push(move_);
                break;
            }
            _ => panic!("This case shoud never happn, check the code")
        }
    }
    moves
}

// fn check_attack_vector(p: &impl PieceCommon, direction: VectorDirection, attack_vectors: &HashMap<VectorDirection, AttackVector>) -> bool {
//     match direction {
//         VectorDirection::UP | VectorDirection::RIGHT => attack_vectors.get(Vec)
//     }
// }

fn line_left(p: &impl PieceCommon, board: &ChessBoard) -> Vec<Move> {
    let pos = p.position();
    let n_travel = pos.1 + 1;
    let pos_trans = |i: i8| {
        (pos.0, pos.1 - i)
    };
    diag_line_generic(p, board, n_travel, pos_trans)
}

fn line_right(p: &impl PieceCommon, board: &ChessBoard) -> Vec<Move> {
    let pos = p.position();
    let n_travel = 8 - pos.1;
    let pos_trans = |i: i8| {
        (pos.0, pos.1 + i)
    };
    diag_line_generic(p, board, n_travel, pos_trans)
}

fn line_up(p: &impl PieceCommon, board: &ChessBoard) -> Vec<Move> {
    let pos = p.position();
    let n_travel = pos.0 + 1;
    let pos_trans = |i: i8| {
        (pos.0 - i, pos.1)
    };
    diag_line_generic(p, board, n_travel, pos_trans)
}

fn line_down(p: &impl PieceCommon, board: &ChessBoard) -> Vec<Move> {
    let pos = p.position();
    let n_travel = 8 - pos.0;
    let pos_trans = |i: i8| {
        (pos.0 + i, pos.1)
    };
    diag_line_generic(p, board, n_travel, pos_trans)
}

fn diag_left_up(p: &impl PieceCommon, board: &ChessBoard) -> Vec<Move> {
    let pos = p.position();
    let n_travel = std::cmp::min(pos.1 + 1, pos.0 + 1);
    let pos_trans = |i: i8| {
        (pos.0 - i, pos.1 - i)
    };
    diag_line_generic(p, board, n_travel, pos_trans)
}

fn diag_right_up(p: &impl PieceCommon, board: &ChessBoard) -> Vec<Move> {
    let pos = p.position();
    let n_travel = std::cmp::min(8 - pos.1, pos.0 + 1);
    let pos_trans = |i: i8| {
        (pos.0 - i, pos.1 + i)
    };
    diag_line_generic(p, board, n_travel, pos_trans)
}

fn diag_left_down(p: &impl PieceCommon, board: &ChessBoard) -> Vec<Move> {
    let pos = p.position();
    let n_travel =  std::cmp::min(pos.1 + 1, 8 - pos.0);
    let pos_trans = |i: i8| {
        (pos.0 + i, pos.1 - i)
    };
    diag_line_generic(p, board, n_travel, pos_trans)
}

fn diag_right_down(p: &impl PieceCommon, board: &ChessBoard) -> Vec<Move> {
    let pos = p.position();
    let n_travel =  std::cmp::min(8 - pos.1, 8 - pos.0);
    let pos_trans = |i: i8| {
        (pos.0 + i, pos.1 + i)
    };
    diag_line_generic(p, board, n_travel, pos_trans)
}

/// 
pub trait PieceCommon {
    fn new(p: Position, color: Color, id: usize) -> Self;

    fn color(&self) -> Color;

    fn position(&self) -> Position;

    fn gen_moves(&self, board: &ChessBoard) -> Vec<Move>;

    fn emoji_repr(&self) -> &'static str;

    fn get_type(&self) -> PieceType;

    fn get_id(&self) -> usize;
}

#[derive(Clone, Debug)]
pub struct Pawn {
    id: usize,
    color: Color,
    pub position: Position,
    pub has_moved: bool,
    pub headstart: bool,
    pin_vector: Option<VectorDirection>
}

impl PieceCommon for Pawn {
    fn new(p: Position, color: Color, id: usize) -> Self {
        Self {
            color: color,
            position: p,
            id: id,
            has_moved: false,
            headstart: false,
            pin_vector: None
        }
    }

    fn color(&self) -> Color {
        self.color.clone()
    }

    fn position(&self) -> Position {
        self.position.clone()
    }

    fn get_id(&self) -> usize {
        self.id
    }

    // Pawn moves are actually the hardest to generate
    fn gen_moves(&self, board: &ChessBoard) -> Vec<Move> {
        let mut pin_updown = false;
        let mut pin_downleft_rightup = false;
        let mut pin_downright_leftup = false;
        match self.pin_vector {
            Some(VectorDirection::LEFT) | Some(VectorDirection::RIGHT) => return vec!(),
            Some(VectorDirection::UP) | Some(VectorDirection::DOWN) => pin_updown = true,
            Some(VectorDirection::DOWN_LEFT) | Some(VectorDirection::UP_RIGHT) => pin_downleft_rightup = true,
            Some(VectorDirection::DOWN_RIGHT) | Some(VectorDirection::UP_LEFT) => pin_downright_leftup = true,
            _ => {}
        }
        let mut moves = vec!();
        let pos = self.position();
        let direction = match &self.color {
            &Color::Black => 1,
            &Color::White => -1
        };

        // The basic move
        let base_move = Move::new(self, board, (pos.0 + direction, pos.1));
        
        let base_move_cond = match &base_move {
            Move::Move(_, _) => true,
            _ => false
        };
        if base_move_cond && !pin_downright_leftup && !pin_downleft_rightup{
            // If the game starts, we can move two squares
            moves.push(base_move);
            if !self.has_moved {
                let second_move = Move::new(self, board, (pos.0 + direction * 2, pos.1));
                if let Move::Move(_, _) = second_move {
                    moves.push(second_move);
                }
            }
        }

        // Taking enemy pieces on the side, en passant is handled here
        let take_left = Move::new_with_enpassant(self, board, (pos.0 + direction, pos.1 + direction));
        let take_right = Move::new_with_enpassant(self, board, (pos.0 + direction, pos.1 - direction));
        if let Move::Take(_, _) = take_left {
            if !pin_downleft_rightup && !pin_updown{
                moves.push(take_left);
            }
        }
        else if let Move::EnPassant(_, _) = take_left {
            if !pin_downleft_rightup && !pin_updown{
                moves.push(take_left);
            }
        }
        if let Move::EnPassant(_, _) = take_right {
            if !pin_downright_leftup && !pin_updown{
                moves.push(take_right);
            }
        }
        else if let Move::Take(_, _) = take_right  {
            if !pin_downright_leftup && !pin_updown{
                moves.push(take_right);
            }
        }
        moves
    }

    fn emoji_repr(&self) -> &'static str {
        match self.color {
            Color::Black => "♟",
            Color::White => "♙"
        }
    }

    fn get_type(&self) -> PieceType {
        PieceType::Pawn
    }
}

#[derive(Clone, Debug)]
pub struct Knight {
    color: Color,
    id: usize,
    position: Position,
    pin_vector: Option<VectorDirection>
}

impl PieceCommon for Knight {
    fn new(p: Position, color: Color, id: usize) -> Self {
        Self {
            color: color,
            id: id,
            position: p,
            pin_vector: None
        }
    }

    fn color(&self) -> Color {
        self.color.clone()
    }

    fn position(&self) -> Position {
        self.position.clone()
    }

    fn gen_moves(&self, board: &ChessBoard) -> Vec<Move> {
        // A pinned knight cannot move
        if self.pin_vector.is_some() {
            return vec!()
        }
        // ad - hoc moves
        vec![
            Move::new(self, board, (self.position.0 - 2, self.position.1 - 1)),
            Move::new(self, board, (self.position.0 - 2, self.position.1 + 1)),
            Move::new(self, board, (self.position.0 + 2, self.position.1 - 1)),
            Move::new(self, board, (self.position.0 + 2, self.position.1 + 1)),
            Move::new(self, board, (self.position.0 - 1, self.position.1 - 2)),
            Move::new(self, board, (self.position.0 - 1, self.position.1 + 2)),
            Move::new(self, board, (self.position.0 + 1, self.position.1 - 2)),
            Move::new(self, board, (self.position.0 + 1, self.position.1 + 2)),
        ]
    }

    fn emoji_repr(&self) -> &'static str {
        match self.color {
            Color::Black => "♞",
            Color::White => "♘"
        }
    }

    fn get_type(&self) -> PieceType {
        PieceType::Knight
    }

    fn get_id(&self) -> usize {
        self.id
    }
}

#[derive(Clone, Debug)]
pub struct Rook {
    color: Color,
    position: Position,
    id: usize,
    has_moved: bool,
    pin_vector: Option<VectorDirection>
}

impl PieceCommon for Rook {
    fn new(p: Position, color: Color, id: usize) -> Self {
        Self {
            color: color,
            id: id,
            position: p,
            has_moved: false,
            pin_vector: None
        }
    }

    fn color(&self) -> Color {
        self.color.clone()
    }

    fn position(&self) -> Position {
        self.position.clone()
    }

    fn gen_moves(&self, board: &ChessBoard) -> Vec<Move> {
        let mut moves = vec!();
        match self.pin_vector {
            Some(VectorDirection::LEFT) | Some(VectorDirection::RIGHT) => {
                moves.extend(line_left(self, board));
                moves.extend(line_right(self, board));
            },
            Some(VectorDirection::DOWN) | Some(VectorDirection::UP) => {
                moves.extend(line_up(self, board));
                moves.extend(line_down(self, board));
            },
            _ => {
                moves.extend(line_left(self, board));
                moves.extend(line_right(self, board));
                moves.extend(line_up(self, board));
                moves.extend(line_down(self, board));
            }
        }
        moves
    }
    
    fn emoji_repr(&self) -> &'static str {
        match self.color {
            Color::Black => "♜",
            Color::White => "♖" 
        }
    }

    fn get_type(&self) -> PieceType {
        PieceType::Rook
    }

    fn get_id(&self) -> usize {
        self.id
    }
}

#[derive(Clone, Debug)]
pub struct Queen {
    color: Color,
    id: usize,
    position: Position,
    pub pin_vector: Option<VectorDirection>
}

impl PieceCommon for Queen {
    fn new(p: Position, color: Color, id: usize) -> Self {
        Self {
            color: color,
            id: id,
            position: p,
            pin_vector: None
        }
    }
    fn color(&self) -> Color {
        self.color.clone()
    }

    fn position(&self) -> Position {
        self.position.clone()
    }

    fn gen_moves(&self, board: &ChessBoard) -> Vec<Move> {
        let mut moves = vec!();
        // lines
        match self.pin_vector {
            Some(VectorDirection::DOWN) | Some(VectorDirection::UP) => {
                moves.extend(line_up(self, board));
                moves.extend(line_down(self, board));
            },
            Some(VectorDirection::LEFT) | Some(VectorDirection::RIGHT) => {
                moves.extend(line_left(self, board));
                moves.extend(line_right(self, board));
            },
            Some(VectorDirection::UP_LEFT) | Some(VectorDirection::DOWN_RIGHT) => {
                moves.extend(diag_left_up(self, board));
                moves.extend(diag_right_down(self, board));
            },
            Some(VectorDirection::UP_RIGHT) | Some(VectorDirection::DOWN_LEFT) => {
                moves.extend(diag_left_down(self, board));
                moves.extend(diag_right_up(self, board));
            },
            _ => {
                moves.extend(line_left(self, board));
                moves.extend(line_right(self, board));
                moves.extend(line_up(self, board));
                moves.extend(line_down(self, board));
                // diags
                moves.extend(diag_left_up(self, board));
                moves.extend(diag_right_up(self, board));
                moves.extend(diag_left_down(self, board));
                moves.extend(diag_right_down(self, board));
            }
        }
        moves
    }

    fn emoji_repr(&self) -> &'static str {
        match self.color {
            Color::Black => "♛",
            Color::White => "♕"
        }
    }

    fn get_type(&self) -> PieceType {
        PieceType::Queen
    }

    fn get_id(&self) -> usize {
        self.id
    }
}

#[derive(Clone, Debug)]
pub struct Bishop {
    color: Color,
    id: usize,
    position: Position,
    pin_vector: Option<VectorDirection>
}

impl PieceCommon for Bishop {
    fn new(p: Position, color: Color, id: usize) -> Self {
        Self {
            color: color,
            id: id,
            position: p,
            pin_vector: None
        }
    }
    fn color(&self) -> Color {
        self.color.clone()
    }

    fn position(&self) -> Position {
        self.position.clone()
    }

    fn gen_moves(&self, board: &ChessBoard) -> Vec<Move> {
        let mut moves = vec!();
        match self.pin_vector {
            Some(VectorDirection::DOWN_LEFT) | Some(VectorDirection::UP_RIGHT) => {
                moves.extend(diag_right_up(self, board));
                moves.extend(diag_left_down(self, board));
            }
            Some(VectorDirection::DOWN_RIGHT) | Some(VectorDirection::UP_LEFT) => {
                moves.extend(diag_left_up(self, board));
                moves.extend(diag_right_down(self, board));;
            }
            _ => {
                moves.extend(diag_left_up(self, board));
                moves.extend(diag_right_up(self, board));
                moves.extend(diag_left_down(self, board));
                moves.extend(diag_right_down(self, board));
            }
        }
        moves
    }

    fn emoji_repr(&self) -> &'static str {
        match self.color {
            Color::Black => "♝",
            Color::White => "♗"
        }
    }

    fn get_type(&self) -> PieceType {
        PieceType::Bishop
    }

    fn get_id(&self) -> usize {
        self.id
    }
}


#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub enum VectorDirection {
    UP,
    DOWN,
    LEFT,
    RIGHT,
    UP_LEFT,
    UP_RIGHT,
    DOWN_LEFT,
    DOWN_RIGHT,
}

impl VectorDirection {
    pub fn dir_vec(&self) -> (i8, i8) {
        match self {
            Self::UP => (-1, 0),
            Self::DOWN => (1, 0),
            Self::LEFT => (0, -1),
            Self::RIGHT => (0, 1),
            Self::UP_LEFT => (-1, -1),
            Self::UP_RIGHT => (-1, 1),
            Self::DOWN_LEFT => (1, -1),
            Self::DOWN_RIGHT => (1, 1),
        }  
    }

    fn all_directions() -> Vec<Self> {
        vec![
            Self::UP,
            Self::DOWN,
            Self::LEFT,
            Self::RIGHT,
            Self::UP_LEFT,
            Self::UP_RIGHT,
            Self::DOWN_LEFT,
            Self::DOWN_RIGHT,
        ]
    }
    pub fn is_line(&self) -> bool {
        match self {
            VectorDirection::DOWN | VectorDirection::UP | VectorDirection::LEFT | VectorDirection::RIGHT => true,
            _ => false
        }
    }

    pub fn is_diagonal(&self) -> bool {
        match self {
            VectorDirection::UP_LEFT | VectorDirection::UP_RIGHT | VectorDirection::DOWN_LEFT | VectorDirection::DOWN_RIGHT => true,
            _ => false
        }
    }
}


#[derive(Clone, Debug)]
pub struct AttackVector {
    from: Position,
    to: Position,
    vec_direction: VectorDirection,
    x_bounds: (i8, i8),
    y_bounds: (i8, i8)
}

impl AttackVector {
    pub fn new(from: Position, to: Position, vec_direction: VectorDirection) -> Self {
        let vec =  (to.0 - to.1, from.0 - from.1);
        Self { 
            from: from,
            to: to,
            vec_direction: vec_direction,
            x_bounds: (std::cmp::min(from.0, to.0), std::cmp::max(from.0, to.0)),
            y_bounds: (std::cmp::min(from.1, to.1), std::cmp::max(from.1, to.1))
        }
    }

    pub fn contains_position(&self, p: &Position) -> bool {
        let condi_in = self.x_bounds.0 <= p.0 && p.0 <= self.x_bounds.1 && self.y_bounds.0 <= p.1 && p.1 <= self.y_bounds.1;
        match &self.vec_direction {
            VectorDirection::UP | VectorDirection::DOWN => p.1 == self.from.1 && self.from.0 <= p.0 && condi_in,
            VectorDirection::LEFT | VectorDirection::RIGHT => p.0 == self.from.0 && self.from.1 <= p.0 && condi_in,
            VectorDirection::DOWN_LEFT | VectorDirection::UP_RIGHT => self.from.0 + self.from.1 == p.0 + p.1 && condi_in,
            VectorDirection::DOWN_RIGHT | VectorDirection::UP_LEFT  => self.from.0 - self.from.1 == p.0 - p.1 && condi_in,
        }
    }
}

#[derive(Clone, Debug)]
pub struct King {
    color: Color,
    id: usize,
    position: Position,
    has_moved: bool,
    pub check: bool,
    pub direct_attacks: Vec<HashSet<Position>>
}

impl King {
    fn generic_attack_vector(self_pos: Position, board: &mut ChessBoard, direction: &VectorDirection) {
        let mut current_defender_pos: Position = (-1, -1);
        let mut n_defenders = 0;
        let dst = &board.board[self_pos.0 as usize][self_pos.1 as usize];
        let mut pos = dst.position().unwrap();
        let color = dst.color().unwrap();
        let dir_vec = direction.dir_vec();
        let mut curr_vec = HashSet::new();
        loop {
            pos = (pos.0 + dir_vec.0, pos.1 + dir_vec.1);
            if pos.0 < 0 || pos.0 > 7 || pos.1 < 0 || pos.1 > 7 {
                break;
            }
            curr_vec.insert(pos.clone());
            let dst = &board.board[pos.0 as usize][pos.1 as usize];
            match dst.color() {
                Some(c) if c == color => {
                    n_defenders += 1;
                    current_defender_pos = dst.position().unwrap();
                },
                // A pin
                Some(c) if c != color && n_defenders == 1 => {
                    if dst.can_attack(&direction) {
                        board.board[current_defender_pos.0 as usize][current_defender_pos.1 as usize].set_attack_vector(Some(direction.clone()));
                    }
                    break;
                }
                // A check
                Some(c) if c != color && n_defenders == 0 => {
                    if dst.can_attack(&direction) {
                        match board.board[self_pos.0 as usize][self_pos.1 as usize] {
                            Piece::King(ref mut k) => {
                                k.check = true;
                                k.direct_attacks.push(curr_vec);
                            },
                            _ => panic!("no king")
                        };
                    }
                    break;
                }
                _ if n_defenders > 1 => {
                    break;
                },
                _ => {}
            }
        }
    }

    pub fn self_by_pos<'a>(pos: &Position, board: &'a mut ChessBoard) -> &'a mut Self {
        match board.board[pos.0 as usize][pos.1 as usize] {
            Piece::King(ref mut k) => k,
            _ => panic!("There should always be two kings")
        }
    }

    // if we have a knight attack, we can only take the knight of 
    pub fn check_for_knight_and_pawn_attack(self_pos: &Position, board: &mut ChessBoard) {
        // Pawn checking
        let king_color = King::self_by_pos(self_pos, board).color();
        let direction = if king_color == Color::White {-1} else {1};
        let pos1 = (self_pos.0 + direction, self_pos.1 - 1);
        let pos2 = (self_pos.0 + direction, self_pos.1 + 1);
        for pos in [pos1, pos2] {
            if pos.0 < 0 || pos.0 > 7 || pos.1 < 0 || pos.1 > 7 {
                continue
            }
            match board.board[pos.0 as usize][pos.1 as usize] {
                Piece::Pawn(ref p) if p.color() != king_color => {
                    let king_mut = King::self_by_pos(self_pos, board);
                    king_mut.check = true;
                    king_mut.direct_attacks.push(HashSet::from_iter([pos]));
                    // Impossible to have both a pawn and a knight attack
                    return
                },
                _ => {}
            }
        }
        // Knight checking
        let knight_possible_pos = [
            (self_pos.0 - 2, self_pos.1 - 1),
            (self_pos.0 - 2, self_pos.1 + 1),
            (self_pos.0 + 2, self_pos.1 - 1),
            (self_pos.0 + 2, self_pos.1 + 1),
            (self_pos.0 - 1, self_pos.1 - 2),
            (self_pos.0 - 1, self_pos.1 + 2),
            (self_pos.0 + 1, self_pos.1 - 2),
            (self_pos.0 + 1, self_pos.1 + 2),
        ];
        for pos in knight_possible_pos {
            if pos.0 < 0 || pos.0 > 7 || pos.1 < 0 || pos.1 > 7 {
                continue
            }
            match board.board[pos.0 as usize][pos.1 as usize] {
                Piece::Knight(ref kn) if kn.color() != king_color => {
                    let king_mut = King::self_by_pos(self_pos, board);
                    king_mut.check = true;
                    king_mut.direct_attacks.push(HashSet::from_iter([pos]));
                    break;
                }
                _ => {}
            }
        }
    }

    // Add the pin vectors to pieces
    // We also add directly attacking pieces
    // If the attacking piece is not a knight: we set a vector otherwise, we set 
    // If two attackers --> move king into non controlled territory
    // If 1 attacker && knight --> gen all moves on the vector / take of the offending piece
    pub fn gen_attack_vectors(self_pos: Position, board: &mut ChessBoard) {
        match board.board[self_pos.0 as usize][self_pos.1 as usize] {
            Piece::King(ref mut k) => {
                k.direct_attacks.clear();
                k.check = false;
            },
            _ => panic!("no king")
        };
        Self::check_for_knight_and_pawn_attack(&self_pos, board);
        for direction in VectorDirection::all_directions().iter() {
            Self::generic_attack_vector(self_pos, board, direction)
        }
    }
}

impl PieceCommon for King {
    fn new(p: Position, color: Color, id: usize) -> Self {
        Self {
            color: color,
            id: id,
            position: p,
            has_moved: false,
            check: false,
            direct_attacks: vec!()
        }
    }

    fn color(&self) -> Color {
        self.color.clone()
    }

    fn position(&self) -> Position {
        self.position.clone()
    }

    fn gen_moves(&self, board: &ChessBoard) -> Vec<Move> {
        let filter_moves = |moves: Vec<Move>| -> Vec<Move> {
            moves
                .into_iter()
                .filter(|m| match m {
                    Move::Move(_,  to) => !board.faction.is_controlled(to, &self.color().other()),
                    Move::Take(_, to) => !board.faction.is_controlled(to, &self.color().other()),
                    _ => true
                })
                .collect()
        };
        // Ad-hoc moves
        let mut moves = vec![
            // Lines
            Move::new(self, board, (self.position.0, self.position.1 + 1)),
            Move::new(self, board, (self.position.0, self.position.1 - 1)),
            Move::new(self, board, (self.position.0 + 1, self.position.1)),
            Move::new(self, board, (self.position.0 - 1, self.position.1)),
            // Diags
            Move::new(self, board, (self.position.0 + 1, self.position.1 + 1)),
            Move::new(self, board, (self.position.0 + 1, self.position.1 - 1)),
            Move::new(self, board, (self.position.0 - 1, self.position.1 + 1)),
            Move::new(self, board, (self.position.0 - 1, self.position.1 - 1)),
        ];
        moves = filter_moves(moves);
        if self.has_moved {
            return moves
        }
        let castle_check = |
            step_size: i8,
            mult: i8,
            rook_pos: Position,
            player: &Color,
            opponent_control: &HashSet<Position>
        | {
            let ok_empty = (1..=step_size).all(|i| {
                let pos = (self.position.0, self.position.1 + i * mult);
                //println!("pos: {:?}, is_empty: {}, opponent control: {}", pos, board.board[pos.0 as usize][pos.1 as usize].is_empty(), opponent_control.contains(&pos));
                board.board[pos.0 as usize][pos.1 as usize].is_empty() && !opponent_control.contains(&pos)
            });
            //println!("Condi castle empty: {}, self_moved: {}, rook: {:?}", ok_empty, self.has_moved, player_pieces.get(&rook_pos));
            let dst = &board.board[rook_pos.0 as usize][rook_pos.1 as usize];
            match (ok_empty, self.has_moved, dst) {
                (true, false, Piece::Rook(r)) if !r.has_moved && &r.color() == player => {
                    true
                }
                _ => {
                    false
                }
            }
        };

        match self.color {
            Color::White => {
                // Kingside
                if castle_check(2, 1, (7, 7), &self.color, &board.faction.black_controlled) {
                    moves.push(Move::KingsideCastle(self.color.clone()));
                }
                // queenside
                if castle_check(3, -1, (7, 0), &self.color, &board.faction.black_controlled) {
                    moves.push(Move::QueensideCastle(self.color.clone()));
                }
            },
            Color::Black => {
                // Kingside
                if castle_check(2, -1, (0, 0), &self.color, &board.faction.white_controlled) {
                    moves.push(Move::KingsideCastle(self.color.clone()));
                }
                // queenside
                if castle_check(3, 1, (0, 7), &self.color, &board.faction.white_controlled) {
                    moves.push(Move::QueensideCastle(self.color.clone()));
                }
            }
        }
        moves
    }

    fn emoji_repr(&self) -> &'static str {
        match self.color {
            Color::Black => "♚",
            Color::White => "♔"
        }
    }

    fn get_type(&self) -> PieceType {
        PieceType::King
    }

    fn get_id(&self) -> usize {
        self.id
    }
}
