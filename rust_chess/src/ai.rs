use std::collections::{HashSet, HashMap};

use crate::{chessbord::ChessBoard, piece::{Move, Color, CanPromoteTo, Position, PieceType}, game::GameEngine};
use rand::prelude::*;
use rayon::prelude::*;

pub trait Ai {
    fn play(&mut self, board: &GameEngine) -> Vec<Move>;

    fn eval_position(&self, board: &ChessBoard) -> f64;

    fn new(machine_player: Color) -> Self
    where Self: Sized;

    fn set_depht(&mut self, depht: usize) {
        todo!()
    }
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



fn init_node_eval(is_max: bool) -> f64 {
    if is_max {
        return -f64::INFINITY;
    }
    else {
        return f64::INFINITY;
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
    fn mini_max_iterative_deepening(
        &self,
        engine: &mut GameEngine,
    ) -> (f64, i64) {
        let mut transposition_table = HashMap::new();
        let mut eval = 0.0;
        let is_max= match engine.current_player {
            Color::White => true, // white -> max
            Color::Black => false, // black -> min
        };
        let init_alpha = -f64::INFINITY;
        let init_beta = f64::INFINITY;
        let mut called = 0;
        let mut total_called = 0;
        let mut called_max_depth = 0;
        for i in 1..= self.depth {
            // we actualy don't care about the eval, we just init it to the correct value based on the max_depth - 1 player
            eval = self.mini_max(engine, 0, i, is_max, init_alpha, init_beta, &mut transposition_table, &mut called);
            //println!("eval ({:?}) depht {}: {} | TT size:{} | called: {}", engine.current_player, i, eval, transposition_table.len(), &called);
            total_called += called;
            if i == self.depth {
                called_max_depth += called;
            }
            called = 0;
        }
        (eval, called_max_depth)
    }

    fn mini_max(
        &self,
        engine: &mut GameEngine,
        depht: usize,
        max_depht: usize,
        is_max: bool,
        mut alpha: f64,
        mut beta: f64,
        transposition_table: &mut HashMap<String, (usize, f64)>,
        called: &mut i64
    ) -> f64 {
        *called += 1;
        let curr_board_key = engine.get_board_as_key();
        // If the move is in the transposition table, we return it
        let relative_depht = max_depht - depht;
        let transpo = transposition_table.get(&curr_board_key).cloned();
        match transpo {
            Some((d, v)) if d >= relative_depht => {
               // println!("using ttable !! ");
                return v
            }
            _ => {}
        }
        // If we are at max depht, we return the position evaluation
        if depht == max_depht {
            let eval = self.eval_position(&engine.board);
            //transposition_table.insert((curr_board_key, relative_depht), eval);
            return eval
        }
        
        // Otherwise we keep searching the tree
        let possible_moves = engine.gen_all_moves();

        let mut new_engines: Vec<_> = possible_moves.into_iter().map(|m| {
            let mut new_engine = engine.clone();
            new_engine.play_once(m);
            new_engine
        }).collect();

        // Sketchy move ordering to improve alpha beta pruning
        // new_engines.sort_by(|m1, m2| {
        //     let m1s = transposition_table.get(&m1.get_board_as_key()).unwrap_or(&(0usize, 0.0)).1;
        //     let m2s = transposition_table.get(&m2.get_board_as_key()).unwrap_or(&(0usize, 0.0)).1;
        //     let ord = m1s.total_cmp(&m2s);
        //     match !is_max {
        //         true => ord,
        //         false => ord.reverse()
        //     }
        // });
        
        // alternative end conditions, checkmate or draw
        if new_engines.len() == 0 {
            // If checkmate
            if engine.check {
                return match engine.current_player {
                    Color::Black => f64::INFINITY,
                    Color::White => -f64::INFINITY
                }
            }
            // if draw (pat, no moves available and not in check)
            else {
                return 0.0
            }
        }
        
        let mut curr_val = init_node_eval(is_max);
        for mut new_engine in new_engines {
            new_engine.finish_turn();
            new_engine.prepare_new_turn();
            if is_max {
                let next_eval = self.mini_max(&mut new_engine, depht + 1, max_depht, !is_max, alpha, beta, transposition_table, called);
                if next_eval >= curr_val {
                    curr_val = next_eval;
                }
                if curr_val >= beta {
                    break;
                }
                alpha = std::cmp::max_by(alpha, curr_val, |a, b| a.total_cmp(b));
            }
            else {
                let next_eval = self.mini_max(&mut new_engine, depht + 1, max_depht, !is_max, alpha, beta, transposition_table, called);
                if next_eval <= curr_val {
                    curr_val = next_eval;
                }
                if curr_val <= alpha {
                    break;
                }
                beta = std::cmp::min_by(beta, curr_val, |a, b| a.total_cmp(b));
            }
        }
        match transpo {
            Some((d, _)) if d >= relative_depht => {}
            _ => {
                transposition_table.insert(curr_board_key, (relative_depht, curr_val));
            }
        }
        curr_val
    }
}

impl Ai for MiniMaxAi {
    fn play(&mut self, engine: &GameEngine) -> Vec<Move> {
        let mut ai_moves = vec!();
        let moves_to_evaluate = engine.gen_all_moves();
        if moves_to_evaluate.len() == 0 {
            return ai_moves;
        }
        let t = std::time::Instant::now();
        let scores: Vec<_> = moves_to_evaluate
            .par_iter()
            .map(|m| {
                let mut evaluation_engine = engine.clone();
                evaluation_engine.play_once(m.clone());
                evaluation_engine.finish_turn();
                evaluation_engine.prepare_new_turn();
                let evaluation = self.mini_max_iterative_deepening(&mut evaluation_engine);
                evaluation
            })
            .collect();
        let mult = match self.machine_player {
            Color::Black => -1.0,
            Color::White => 1.0
        };
        let mut total_called = 0;
        let best_move = scores
            .iter()
            .enumerate()
            .fold((0, -f64::INFINITY), |(i_acc, v_acc), (i, (v, c))| {
                total_called += c;
                let vmult = mult * v;
                if vmult > v_acc {
                    (i, vmult)
                }
                // If two moves are equal we randomize
                else if vmult == v_acc {
                    let mut rng = thread_rng();
                    if rng.gen::<f64>() < 0.5 {
                        (i_acc, v_acc)
                    }
                    else {
                        (i, vmult)
                    }
                }
                else {
                    (i_acc, v_acc)
                }
            });
        let elapsed = t.elapsed();
        println!("elapsed: {:?}", elapsed);
        println!("eval ({:?}): {} | n-nodes: {} | elapsed: {}, nodes/s: {}",
            engine.current_player,
            best_move.1,
            total_called,
            elapsed.as_secs(),
            total_called as f64 / elapsed.as_secs_f64(),
        );
        ai_moves.push(moves_to_evaluate[best_move.0].clone());
        ai_moves
    }

    fn eval_position(&self, board: &ChessBoard) -> f64 {
        let mut eval = 0.0;
        for piece in board.faction.black_pieces.values() {
            let ptype = board.board[piece.0 as usize][piece.1 as usize].get_type().unwrap();
            eval -= self.piece_values.get(&ptype).unwrap();
        }
        for piece in board.faction.white_pieces.values() {
            let ptype = board.board[piece.0 as usize][piece.1 as usize].get_type().unwrap();
            eval += self.piece_values.get(&ptype).unwrap();
        }
        eval
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

    fn set_depht(&mut self, depht: usize) {
        self.depth = depht;
    }
}