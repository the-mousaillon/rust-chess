use crate::{chessbord::ChessBoard, piece::{Color, Move}, game::GameEngine};

use super::piece::{PieceType};

pub struct Zobrist {
    pub table: [[u64; 12]; 64],
    pub black_to_move: u64
}

impl Zobrist {
    pub fn new() -> Self {
        let mut table = [[0u64; 12]; 64];
        for i in 0..64 {
            for j in 0..12 {
                table[i][j] = rand::random();
            }
        }
        Self { table: table, black_to_move: rand::random() }
    }

    pub fn hash(&self, board: &ChessBoard, player: &Color) -> u64 {
        let mut zob_hash = match player {
            Color::Black => 0u64 ^ self.black_to_move,
            Color::White => 0u64,
        };
        for (_, (x, y)) in &board.faction.white_pieces {
            let flat_idx = *x as usize * 8 + *y as usize;
            let zob_piece_id = board.board[*x as usize][*y as usize].get_zobrist_id();
            let zob = self.table[flat_idx][zob_piece_id as usize];
            zob_hash = zob_hash ^ zob;
        }
        for (_, (x, y)) in &board.faction.black_pieces {
            let flat_idx = *x as usize * 8 + *y as usize;
            let zob_piece_id = board.board[*x as usize][*y as usize].get_zobrist_id();
            let zob = self.table[flat_idx][zob_piece_id as usize];
            zob_hash = zob_hash ^ zob;
        }
        zob_hash
    }

    pub fn zob_id_from_pos_and_id(&self, pos: &(i8, i8), piece_zob_id: u64) -> u64 {
        let flat_idx = pos.0 as usize * 8 + pos.1 as usize;
        self.table[flat_idx][piece_zob_id as usize]
    }

    pub fn update_hash(&self, prev_hash: u64, board: &ChessBoard, play: &Move, curr_player: &Color) -> u64 {
        let mut next_hash = prev_hash;
        // We apply a move so we xor in/out the color marker
        next_hash = next_hash ^ self.black_to_move;
        
        match play {
            Move::EnPassant(from, to) => {
                let (from_c, to_c) = if from.0 > to.0 {(0, 6)} else {(6, 0)};
                let zob_idx_from = PieceType::Pawn.get_zobrist_id() + from_c;
                let zob_idx_to =  PieceType::Pawn.get_zobrist_id() + to_c;
                let taken_pawn_pos = (to.1, from.0);
                let zob_from = self.zob_id_from_pos_and_id(from, zob_idx_from);
                let zob_to = self.zob_id_from_pos_and_id(&taken_pawn_pos, zob_idx_to);
                next_hash = next_hash ^ zob_from ^ zob_to;
            },
            Move::KingsideCastle(c) => {
                let (add, king_pos_from, king_pos_to, rook_pos_from, rook_pos_to) = match c {
                    Color::Black => (6, (0, 4), (0, 6), (0, 7), (0, 5)),
                    Color::White => (0, (7, 4), (7, 6), (7, 7), (7, 5)),
                };
                let rook_zob = PieceType::Rook.get_zobrist_id() + add;
                let king_zob= PieceType::King.get_zobrist_id() + add;
                let king_from_zid = self.zob_id_from_pos_and_id(&king_pos_from, king_zob);
                let king_to_zid = self.zob_id_from_pos_and_id(&king_pos_to, king_zob);
                let rook_from_zid = self.zob_id_from_pos_and_id(&rook_pos_from, rook_zob);
                let rook_to_zid = self.zob_id_from_pos_and_id(&rook_pos_to, rook_zob);
                // Applying the zids
                next_hash = next_hash ^ king_from_zid ^ rook_from_zid ^ king_to_zid ^ rook_to_zid;
            },
            Move::Move(from, to) => {
                let piece_ref_from = &board.board[from.0 as usize][from.1 as usize];
                let piece_ref_to = &board.board[to.0 as usize][to.1 as usize];
                let zob_id_from = piece_ref_from.get_zobrist_id();
                // Xoring out the from
                next_hash = next_hash ^ self.zob_id_from_pos_and_id(from, zob_id_from);
                // Xoring out the to if not empty
                if !piece_ref_to.is_empty() {
                    let zob_id_to = piece_ref_to.get_zobrist_id();
                    next_hash = next_hash ^ self.zob_id_from_pos_and_id(to, zob_id_to);
                }
                // Xoring in the from
                next_hash = next_hash ^ self.zob_id_from_pos_and_id(to, zob_id_from);
            },
            Move::Promote(from, to_type) => {
                let mut add = 0;
                let piece_ref = &board.board[from.0 as usize][from.1 as usize];
                if piece_ref.color().unwrap() == Color::Black {
                    add = 6;
                };
                let piece_init_zob = self.zob_id_from_pos_and_id(from, piece_ref.get_zobrist_id());
                let piece_new_zob = self.zob_id_from_pos_and_id(from, to_type.get_zobrist_id() + add);
                next_hash = next_hash ^ piece_init_zob ^ piece_new_zob;
            },
            Move::QueensideCastle(c) => {
                let (add, king_pos_from, king_pos_to, rook_pos_from, rook_pos_to) = match c {
                    Color::Black => (6, (0, 4), (0, 2), (0, 0), (0, 3)),
                    Color::White => (0, (7, 4), (7, 2), (7, 0), (7, 3)),
                };
                let rook_zob = PieceType::Rook.get_zobrist_id() + add;
                let king_zob= PieceType::King.get_zobrist_id() + add;
                let king_from_zid = self.zob_id_from_pos_and_id(&king_pos_from, king_zob);
                let king_to_zid = self.zob_id_from_pos_and_id(&king_pos_to, king_zob);
                let rook_from_zid = self.zob_id_from_pos_and_id(&rook_pos_from, rook_zob);
                let rook_to_zid = self.zob_id_from_pos_and_id(&rook_pos_to, rook_zob);
                // Applying the zids
                next_hash = next_hash ^ king_from_zid ^ rook_from_zid ^ king_to_zid ^ rook_to_zid;
            },
            Move::Take(from, to) => {
                let piece_ref_from = &board.board[from.0 as usize][from.1 as usize];
                let piece_ref_to = &board.board[to.0 as usize][to.1 as usize];
                let zob_id_from = piece_ref_from.get_zobrist_id();
                let zob_id_to = piece_ref_to.get_zobrist_id();
                // Xoring out the from
                next_hash = next_hash ^ self.zob_id_from_pos_and_id(from, zob_id_from);
                // Xoring out the to (never empty so no ckeck needed)
                next_hash = next_hash ^ self.zob_id_from_pos_and_id(to, zob_id_to);
                // Xoring in the from
                next_hash = next_hash ^ self.zob_id_from_pos_and_id(from, zob_id_from);
            },
            _ => {}
        }
        next_hash
    }
}


#[test]
fn test_zobrist() {
    let zob = Zobrist::new();
    let mut engine = GameEngine::new();
    engine.board.pprint();
    let zob_init = zob.hash(&engine.board, &engine.current_player);
    println!("Zobrist (init): {}", zob_init);
    let mo = Move::Move((0, 0), (3, 0));
    let zob_opti = zob.update_hash(zob_init, &engine.board, &mo, &engine.current_player);
    engine.play_once(mo.clone());
    engine.finish_turn();
    engine.prepare_new_turn();
    engine.board.pprint();
    println!("curr player: {:?}", engine.current_player);
    println!("Zobrist (base): {}", zob.hash(&engine.board, &engine.current_player));
    println!("Zobrist (optimized): {}", zob_opti);
}