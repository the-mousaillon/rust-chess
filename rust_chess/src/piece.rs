use super::chessbord::ChessBoard;

#[derive(Clone, Debug)]
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

    pub fn pprint(&self) {
        let emoj = self.emoji();
        print!("{}", emoj);
    }

    pub fn emoji(&self) -> &'static str {
        let empty = " ";
        multi_match!(self, pprint_piece, empty)
    }
}

pub type Position = (u8, u8);

#[derive(Clone, Debug)]
pub enum Move {
    Move(Position, Position),
    Take(Position, Position),
    EnPassant(Position, Position),
    KingsideCastle,
    QueensideCastle,
}

pub trait PieceCommon {
    fn new(p: Position, color: Color) -> Self;

    fn color(&self) -> Color;

    fn gen_moves(&self, board: ChessBoard) -> Vec<Move>;

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

    fn gen_moves(&self, board: ChessBoard) -> Vec<Move> {
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

    fn gen_moves(&self, board: ChessBoard) -> Vec<Move> {
        vec![]
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

    fn gen_moves(&self, board: ChessBoard) -> Vec<Move> {
        vec![]
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

    fn gen_moves(&self, board: ChessBoard) -> Vec<Move> {
        vec![]
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

    fn gen_moves(&self, board: ChessBoard) -> Vec<Move> {
        vec![]
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

    fn gen_moves(&self, board: ChessBoard) -> Vec<Move> {
        vec![]
    }

    fn emoji_repr(&self) -> &'static str {
        match self.color {
            Color::Black => "♚",
            Color::White => "♔"
        }
    }
}
