use std::collections::HashSet;

use crate::{chessbord::ChessBoard, piece::{Move, Color, CanPromoteTo, Position}, game::GameEngine};
use rand::prelude::*;

pub trait Ai {
    fn play(&mut self, board: &GameEngine) -> Vec<Move>;

    fn eval_position(&mut self, board: &ChessBoard) -> f64;

    fn new(machine_player: Color) -> Self
    where Self: Sized;
}


pub struct DummyRandomIA {
    machine_player: Color,
    rng: ThreadRng
}

impl Ai for DummyRandomIA {
    fn new(machine_player: Color) -> Self {
        Self {
            machine_player: machine_player,
            rng: rand::thread_rng()
        }    
    }

    fn play(&mut self, engine: &GameEngine) -> Vec<Move> {
        let mut chosen_moves = vec!();
        let moves = engine.gen_all_moves();
        if moves.len() == 0  {
            return vec!()
        };
        let random_selection: f64 = self.rng.gen();
        let random_selection = (random_selection * moves.len() as f64) as usize;
        let move_tmp = moves[random_selection].clone();
        let is_promotable = move_tmp.is_promotable(&engine.board);
        let move_tmp_to = move_tmp.to().unwrap();
        chosen_moves.push(move_tmp);
        // If the move leads to a promotion, we need to return two moves
        if  is_promotable {
            let promotions = [
                Move::Promote(move_tmp_to, CanPromoteTo::Rook),
                Move::Promote(move_tmp_to, CanPromoteTo::Bishop),
                Move::Promote(move_tmp_to, CanPromoteTo::Knight),
                Move::Promote(move_tmp_to, CanPromoteTo::Queen)
            ];
            let random_selection: f64 = self.rng.gen();
            let random_selection = (random_selection * promotions.len() as f64) as usize;
            chosen_moves.push(promotions[random_selection].clone());
        }
        chosen_moves
    }

    fn eval_position(&mut self, board: &ChessBoard) -> f64 {
        todo!()
    }
}