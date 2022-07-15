use super::chessbord::ChessBoard;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Color {
    Black,
    White,
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
    };
}

impl Piece {
    pub fn color(&self) -> Option<Color> {
        multi_match!(self, get_piece_color, None)
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
}

pub type Position = (u8, u8);


//// MOVES 
#[derive(Clone, Debug)]
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
        let dst = &board.board[to.0 as usize][to.1 as usize];
        // If we try to move in our own piece, the move is invalid
        if !dst.is_empty() && (dst.color().unwrap() == p.color()) {
            return Move::Invalid
        }
        // Otherwise we either move or take
        match dst {
            Piece::Empty => Self::Move(p.position(), to),
            p_ => Move::Take(p.position(), to),
        }
    }
}

fn diag_line_generic(p: &impl PieceCommon, board: &ChessBoard, n_travel: u8, pos_trans: impl Fn(u8) -> Position) -> Vec<Move> {
    let mut moves = vec!();
    for i in 0..n_travel {
        let to = pos_trans(i);
        let move_ = Move::new(p, board, to);
        match move_ {
            Move::Move(_, _) =>{
                moves.push(move_);
            },
            Move::Take(_, _) => {
                moves.push(move_);
                break
            },
            Move::Invalid => {
                break
            }
            _ => panic!("This case shoud never happn, check the code")
        }
    }
    moves
}

fn line_left(p: &impl PieceCommon, board: &ChessBoard) -> Vec<Move> {
    let pos = p.position();
    let n_travel = pos.1;
    let pos_trans = |i: u8| {
        (pos.0, pos.1 - i)
    };
    diag_line_generic(p, board, n_travel, pos_trans)
}

fn line_right(p: &impl PieceCommon, board: &ChessBoard) -> Vec<Move> {
    let pos = p.position();
    let n_travel = pos.1;
    let pos_trans = |i: u8| {
        (pos.0, pos.1 + i)
    };
    diag_line_generic(p, board, n_travel, pos_trans)
}

fn line_up(p: &impl PieceCommon, board: &ChessBoard) -> Vec<Move> {
    let pos = p.position();
    let n_travel = pos.1;
    let pos_trans = |i: u8| {
        (pos.0 - i, pos.1)
    };
    diag_line_generic(p, board, n_travel, pos_trans)
}

fn line_down(p: &impl PieceCommon, board: &ChessBoard) -> Vec<Move> {
    let pos = p.position();
    let n_travel = pos.1;
    let pos_trans = |i: u8| {
        (pos.0 + 1, pos.1)
    };
    diag_line_generic(p, board, n_travel, pos_trans)
}

fn diag_left_up(p: &impl PieceCommon, board: &ChessBoard) -> Vec<Move> {
    let pos = p.position();
    let n_travel = pos.1;
    let pos_trans = |i: u8| {
        (pos.0 - i, pos.1 - i)
    };
    diag_line_generic(p, board, n_travel, pos_trans)
}

fn diag_right_up(p: &impl PieceCommon, board: &ChessBoard) -> Vec<Move> {
    let pos = p.position();
    let n_travel = pos.1;
    let pos_trans = |i: u8| {
        (pos.0 - i, pos.1 + i)
    };
    diag_line_generic(p, board, n_travel, pos_trans)
}

fn diag_left_down(p: &impl PieceCommon, board: &ChessBoard) -> Vec<Move> {
    let pos = p.position();
    let n_travel = pos.1;
    let pos_trans = |i: u8| {
        (pos.0 + i, pos.1 - i)
    };
    diag_line_generic(p, board, n_travel, pos_trans)
}

fn diag_right_down(p: &impl PieceCommon, board: &ChessBoard) -> Vec<Move> {
    let pos = p.position();
    let n_travel = pos.1;
    let pos_trans = |i: u8| {
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
}

#[derive(Clone, Debug)]
pub struct Pawn {
    color: Color,
    position: Position,
}

impl PieceCommon for Pawn {
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
        vec![]
    }

    fn emoji_repr(&self) -> &'static str {
        match self.color {
            Color::Black => "♟",
            Color::White => "♙"
        }
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
}

#[derive(Clone, Debug)]
pub struct Rook {
    color: Color,
    position: Position,
}

impl PieceCommon for Rook {
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
}

#[derive(Clone, Debug)]
pub struct King {
    color: Color,
    position: Position,
}

impl PieceCommon for King {
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
        // Ad-hoc moves
        vec![
            // Lines
            Move::new(self, board, (self.position.0, self.position.1 + 1)),
            Move::new(self, board, (self.position.0, self.position.1 - 1)),
            Move::new(self, board, (self.position.0 + 1, self.position.1)),
            Move::new(self, board, (self.position.0 + 1, self.position.1)),
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
}
