use std::collections::HashMap;

use serde::{Serialize, Deserialize};

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
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
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
    pub fn color(&self) -> Option<Color> {
        multi_match!(self, get_piece_color, None)
    }

    pub fn get_type(&self) -> Option<PieceType> {
        multi_match!(self, get_piece_type, None)
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
                p.headstart = false;
                if !p.has_moved && (p.position.0 - pos.0).abs() == 2 {
                    println!("HEADSTART");
                    p.headstart = true
                }
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
    pub fn webapp_repr(&self) -> CellRepr {
        let color = self.color();
        match self {
            Piece::King(p) => CellRepr {
                piece: PieceRepr {
                    idx: Some(0),
                    color: color.map(|c| c.webapp_repr()),
                    name: Some("KING".into())
                },
                threatened: false
            },
            Piece::Queen(p) => CellRepr {
                piece: PieceRepr {
                    idx: Some(1),
                    color: color.map(|c| c.webapp_repr()),
                    name: Some("QUEEN".into())
                },
                threatened: false
            },
            Piece::Rook(p) => CellRepr {
                piece: PieceRepr {
                    idx: Some(2),
                    color: color.map(|c| c.webapp_repr()),
                    name: Some("ROOK".into())
                },
                threatened: false
            },
            Piece::Bishop(p) => CellRepr {
                piece: PieceRepr {
                    idx: Some(3),
                    color: color.map(|c| c.webapp_repr()),
                    name: Some("BISHOP".into())
                },
                threatened: false
            },
            Piece::Knight(p) => CellRepr {
                piece: PieceRepr {
                    idx: Some(4),
                    color: color.map(|c| c.webapp_repr()),
                    name: Some("KNIGHT".into())
                },
                threatened: false
            },
            Piece::Pawn(p) => CellRepr {
                piece: PieceRepr {
                    idx: Some(5),
                    color: color.map(|c| c.webapp_repr()),
                    name: Some("PAWN".into())
                },
                threatened: false
            },
            Piece::Empty => CellRepr {
                piece: PieceRepr {
                    idx: Some(10),
                    color: color.map(|c| c.webapp_repr()),
                    name: Some("EMPTY".into())
                },
                threatened: false
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
    Invalid
}

impl Move {
    // This function should be called only for move or take moves. Castling and en passant would make this function pannic
    pub fn new(p: &impl PieceCommon, board: &ChessBoard, to: Position) -> Self {
        if to.0 < 0 || to.0 > 7 || to.1 < 0 || to.1 > 7 {
            return Move::Invalid
        }
        let dst = &board.board[to.0 as usize][to.1 as usize];
        // If we try to move in our own piece, the move is invalid
        println!("dst: {:?}", dst);
        if !dst.is_empty() && (dst.color().unwrap() == p.color()) {
            return Move::Invalid
        }
        // Otherwise we either move or take
        match dst {
            Piece::Empty => Self::Move(p.position(), to),
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
            _ => None
        }
    }

    pub fn is_valid(&self) -> bool {
        match self {
            Self::Invalid => false,
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
                println!("MOVE");
                moves.push(move_);
            },
            Move::Take(_, _) => {
                println!("TAKE");
                moves.push(move_);
                break
            },
            Move::Invalid => {
                println!("INVALID");
                break
            }
            _ => panic!("This case shoud never happn, check the code")
        }
    }
    moves
}

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
    fn new(p: Position, color: Color) -> Self;

    fn color(&self) -> Color;

    fn position(&self) -> Position;

    fn gen_moves(&self, board: &ChessBoard) -> Vec<Move>;

    fn emoji_repr(&self) -> &'static str;

    fn get_type(&self) -> PieceType;
}

#[derive(Clone, Debug)]
pub struct Pawn {
    color: Color,
    position: Position,
    has_moved: bool,
    headstart: bool
}

impl PieceCommon for Pawn {
    fn new(p: Position, color: Color) -> Self {
        Self {
            color: color,
            position: p,
            has_moved: false,
            headstart: false
        }
    }

    fn color(&self) -> Color {
        self.color.clone()
    }

    fn position(&self) -> Position {
        self.position.clone()
    }

    // Pawn moves are actually the hardest to generate
    fn gen_moves(&self, board: &ChessBoard) -> Vec<Move> {
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
        if base_move_cond {
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
        println!("Take left: {:?}, take right: {:?}", take_left, take_right);
        if let Move::Take(_, _) = take_left {
            moves.push(take_left);
        }
        else if let Move::EnPassant(_, _) = take_left {
            moves.push(take_left);
        }
        if let Move::EnPassant(_, _) = take_right {
            moves.push(take_right);
        }
        else if let Move::Take(_, _) = take_right  {
            moves.push(take_right);
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
    position: Position,
}

impl PieceCommon for Knight {
    fn new(p: Position, color: Color) -> Self {
        Self {
            color: color,
            position: p,
        }
    }

    fn color(&self) -> Color {
        self.color.clone()
    }

    fn position(&self) -> Position {
        self.position.clone()
    }

    fn gen_moves(&self, board: &ChessBoard) -> Vec<Move> {
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
}

#[derive(Clone, Debug)]
pub struct Rook {
    color: Color,
    position: Position,
    has_moved: bool
}

impl PieceCommon for Rook {
    fn new(p: Position, color: Color) -> Self {
        Self {
            color: color,
            position: p,
            has_moved: false
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
        moves.extend(line_left(self, board));
        moves.extend(line_right(self, board));
        moves.extend(line_up(self, board));
        moves.extend(line_down(self, board));
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
}

#[derive(Clone, Debug)]
pub struct Queen {
    color: Color,
    position: Position,
}

impl PieceCommon for Queen {
    fn new(p: Position, color: Color) -> Self {
        Self {
            color: color,
            position: p,
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
        moves.extend(line_left(self, board));
        moves.extend(line_right(self, board));
        moves.extend(line_up(self, board));
        moves.extend(line_down(self, board));
        // diags
        moves.extend(diag_left_up(self, board));
        moves.extend(diag_right_up(self, board));
        moves.extend(diag_left_down(self, board));
        moves.extend(diag_right_down(self, board));
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
}

#[derive(Clone, Debug)]
pub struct Bishop {
    color: Color,
    position: Position,
}

impl PieceCommon for Bishop {
    fn new(p: Position, color: Color) -> Self {
        Self {
            color: color,
            position: p,
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
        moves.extend(diag_left_up(self, board));
        moves.extend(diag_right_up(self, board));
        moves.extend(diag_left_down(self, board));
        moves.extend(diag_right_down(self, board));
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
    vec_direction: VectorDirection
}

impl AttackVector {
    pub fn new(from: Position, to: Position, vec_direction: VectorDirection) -> Self {
        let vec =  (to.0 - to.1, from.0 - from.1);
        Self { 
            from: from,
            to: to,
            vec_direction: vec_direction
        }
    }
}

#[derive(Clone, Debug)]
pub struct King {
    color: Color,
    position: Position,
    has_moved: bool
}

impl King {

    fn generic_attack_vector(&self, board: &ChessBoard, direction: &VectorDirection) -> Option<AttackVector> {
        let mut current_defender: &Piece;
        let mut n_defenders = 0;
        let mut pos = self.position();
        let dir_vec = direction.dir_vec();
        loop {
            pos = (pos.0 + dir_vec.0, pos.1 + dir_vec.1);
            if pos.0 < 0 || pos.0 > 7 || pos.1 < 0 || pos.1 > 7 {
                return None;
            }
            let dst = &board.board[pos.0 as usize][pos.1 as usize];
            match dst.color() {
                Some(c) if c == self.color => {
                    n_defenders += 1;
                    current_defender = dst
                },
                Some(c) if c != self.color => {
                    if dst.can_attack(&direction) {
                        let attack_vector = AttackVector::new(self.position, dst.position().unwrap(), direction.clone());
                        return Some(attack_vector)
                    }
                    else {
                        return None
                    }
                }
                _ => {}
            }
            if n_defenders > 1 {
                return None
            }
        }
    }

    pub fn gen_attack_vectors(&self, board: &ChessBoard) -> HashMap<Position, AttackVector> {
        let mut hm_attacks = HashMap::new();
        for direction in VectorDirection::all_directions().iter() {
            if let Some(av) = self.generic_attack_vector(board, direction) {
                hm_attacks.insert(av.to, av);
            }
        }
        hm_attacks
    }
}

impl PieceCommon for King {
    fn new(p: Position, color: Color) -> Self {
        Self {
            color: color,
            position: p,
            has_moved: false
        }
    }

    fn color(&self) -> Color {
        self.color.clone()
    }

    fn position(&self) -> Position {
        self.position.clone()
    }

    fn gen_moves(&self, board: &ChessBoard) -> Vec<Move> {
        // Ad-hoc moves
        vec![
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
        ]
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
}
