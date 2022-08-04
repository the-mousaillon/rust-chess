use std::collections::{HashSet, HashMap};

use crate::{chessbord::ChessBoard, piece::{Move, Color, CanPromoteTo, Position, PieceType}, game::GameEngine};
use rand::prelude::*;
use rayon::prelude::*;

pub trait Ai {
    fn play(&mut self, board: &GameEngine) -> Vec<Move>;

    fn eval_position(&self, board: &ChessBoard) -> f64;

    fn new(machine_player: Color) -> Self
    where Self: Sized;
}


// The most stupid ai, playing randomly
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

    fn eval_position(&self, board: &ChessBoard) -> f64 {
        todo!()
    }
}


// The simplest "smart" ai, choosing the best move at depht 1 with piece value heuristic. Meaning its a very aggressive AI
pub struct BestPlayDephtOneAi {
    machine_player: Color,
    piece_values: HashMap<PieceType, f64>
}

impl Ai for BestPlayDephtOneAi {
    fn new(machine_player: Color) -> Self {
        Self {
            machine_player: machine_player,
            piece_values: HashMap::from_iter([
                (PieceType::Pawn, 1.0),
                (PieceType::Knight, 3.0),
                (PieceType::Bishop, 3.2),
                (PieceType::Rook, 5.0),
                (PieceType::Queen, 9.0),
                (PieceType::King, 100000000000.0)
            ])
        }    
    }

    fn play(&mut self, engine: &GameEngine) -> Vec<Move> {
        let mut chosen_moves = vec!();
        let moves = engine.gen_all_moves();
        let scores: Vec<f64> = moves.par_iter().map(|m| {
            let mut tmp_engine = engine.clone();
            let can_promote = tmp_engine.play_once(m.clone());
            // we will se later for the promotion
            let position_evaluation = self.eval_position(&tmp_engine.board);
            position_evaluation
        })
        .collect();
        let best_move = scores
            .iter()
            .enumerate()
            .fold((0, -f64::INFINITY), |(i_acc, v_acc), (i, v)| {
                if v > &v_acc {
                    (i, *v)
                }
                else {
                    (i_acc, v_acc)
                }
            });
        chosen_moves.push(moves[best_move.0].clone());
        chosen_moves
    }

    fn eval_position(&self, board: &ChessBoard) -> f64 {
        let mut eval = 0.0;
        let white_multi = match self.machine_player {
            Color::Black => -1.0,
            Color::White => 1.0
        };
        for piece in board.faction.black_pieces.values() {
            let ptype = board.board[piece.0 as usize][piece.1 as usize].get_type().unwrap();
            eval += -white_multi * self.piece_values.get(&ptype).unwrap();
        }
        for piece in board.faction.white_pieces.values() {
            let ptype = board.board[piece.0 as usize][piece.1 as usize].get_type().unwrap();
            eval += white_multi * self.piece_values.get(&ptype).unwrap();
        }
        eval
    }
}


// The classic minimax ai, much more powerfull ai
pub struct MiniMaxAi {
    machine_player: Color,
    depth: usize,
    piece_values: HashMap<PieceType, f64>
}

impl MiniMaxAi {
    // Handling promotion will suck
    fn min_max(&self, engine: &GameEngine, propagated_val: Option<f64>, depht: usize, is_max: bool) -> f64 {
        // If we are at max depht, we return the position evaluation
        if depht == self.depth {
            return self.eval_position(&engine.board)
        }

        let mut curr_eval: f64 = match is_max {
            true => -f64::INFINITY,
            false => f64::INFINITY,
        };

        let mut new_propagated_val = None;
        
        // Otherwise we keep searching the tree
        let possible_moves = engine.gen_all_moves();
        
        if possible_moves.len() == 0 {
            // If checkmate
            if engine.check {
                return match is_max {
                    true => -f64::INFINITY,
                    false => f64::INFINITY
                }
            }
            // if draw
            else {
                return 0.0
            }
        }
        for m in possible_moves {
            let mut engine_for_move = engine.clone();
            println!("move: {:?}", m);
            engine_for_move.play_once(m);
            engine_for_move.finish_turn();
            engine_for_move.prepare_new_turn();
            let position_evaluation = self.min_max(&engine_for_move, new_propagated_val, depht + 1, !is_max);
            match (is_max, position_evaluation > curr_eval, propagated_val) {
                (true, _, Some(v)) if v < curr_eval => {
                    // alpha-beta pruning
                    return propagated_val.unwrap()
                },
                (false, _, Some(v)) if v < curr_eval => {
                    // alpha-beta pruning
                    return propagated_val.unwrap()
                }, 
                (true, true, _) => {
                    // keep exploring, max player
                    curr_eval = position_evaluation;
                    new_propagated_val = Some(curr_eval);
                }
                (false, false, _) => {
                    // keep exploring, min player
                    curr_eval = position_evaluation;
                    new_propagated_val = Some(curr_eval);
                },
                _ => {}
            }

        }
        curr_eval
    }
}

impl Ai for MiniMaxAi {
    fn play(&mut self, engine: &GameEngine) -> Vec<Move> {
        let mut ai_moves = vec!();
        let moves_to_evaluate = engine.gen_all_moves();
        if moves_to_evaluate.len() == 0 {
            return ai_moves;
        }
        let scores: Vec<_> = moves_to_evaluate
            .par_iter()
            .map(|m| {
                let mut evaluation_engine = engine.clone();
                evaluation_engine.play_once(m.clone());
                evaluation_engine.finish_turn();
                evaluation_engine.prepare_new_turn();
                let evaluation = self.min_max(&evaluation_engine, None, 1, false);
                evaluation
            })
            .collect();

        let best_move = scores
            .iter()
            .enumerate()
            .fold((0, -f64::INFINITY), |(i_acc, v_acc), (i, v)| {
                if v > &v_acc {
                    (i, *v)
                }
                else {
                    (i_acc, v_acc)
                }
            });
        ai_moves.push(moves_to_evaluate[best_move.0].clone());
        ai_moves
    }

    fn eval_position(&self, board: &ChessBoard) -> f64 {
        let mut eval = 0.0;
        let multi = match self.machine_player {
            Color::Black => -1.0,
            Color::White => 1.0
        };
        for piece in board.faction.black_pieces.values() {
            let ptype = board.board[piece.0 as usize][piece.1 as usize].get_type().unwrap();
            eval -= self.piece_values.get(&ptype).unwrap();
        }
        for piece in board.faction.white_pieces.values() {
            let ptype = board.board[piece.0 as usize][piece.1 as usize].get_type().unwrap();
            eval += self.piece_values.get(&ptype).unwrap();
        }
        multi * eval
    }

    fn new(machine_player: Color) -> Self {
        Self {
            machine_player: machine_player,
            piece_values: HashMap::from_iter([
                (PieceType::Pawn, 1.0),
                (PieceType::Knight, 3.0),
                (PieceType::Bishop, 3.2),
                (PieceType::Rook, 5.0),
                (PieceType::Queen, 9.0),
                (PieceType::King, 100000000000.0)
            ]),
            depth: 3
        }    
    }
}