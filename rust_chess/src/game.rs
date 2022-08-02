use std::collections::{HashMap, HashSet};

use actix::Message;
use serde::{Serialize, Deserialize};

use crate::{
    chessbord::{ChessBoard, WebappRepr, apply_markers},
    piece::{Color, Move, Piece, Position, King, PieceType, CanPromoteTo}, ai::Ai,
};

#[derive(Clone, Debug)]
pub struct GameEngine {
    pub board: ChessBoard,
    pub board_history: Vec<ChessBoard>,
    pub current_player: Color,
    pub current_history_offset: usize,
    pub turn: usize,
    pub promotion_opt: Option<(i8, i8)>,
    pub check: bool,
    pub attack_vector: Vec<HashSet<Position>>
}

impl GameEngine {
    pub fn new() -> Self {
        Self {
            board: ChessBoard::new_default(),
            board_history: vec!(),
            current_player: Color::White,
            current_history_offset: 0,
            turn: 0,
            promotion_opt: None,
            check: false,
            attack_vector: vec!()
        }
    }

    pub fn get_moves_for_piece(&self, pos: &Position) -> HashMap<Position, Move> {
        let dst = &self.board.board[pos.0 as usize][pos.1 as usize];
        dst.gen_moves_check(&self.board, self.check, &self.attack_vector)
            .into_iter()
            .map(|m| (m.to(), m))
            .filter(|(to, _)| to.is_some())
            .map(|(to, m)| (to.unwrap(), m))
            .collect()
    }

    pub fn rollback(&mut self) {
        self.turn -= 1;
        let board_rb = self.board_history.pop().unwrap();
        self.board = board_rb;
    }

    pub fn to_webapp(&self) -> WebappRepr {
        self.board.to_webapp()
    }

    pub fn prepare_new_turn(&mut self) {
        let king_pos = self.board.locate_king(&self.current_player);
        King::gen_attack_vectors(king_pos, &mut self.board);
        let king = match self.board.board[king_pos.0 as usize][king_pos.1 as usize] {
            Piece::King(ref k) => k,
            _ => panic!("No king found")
        };
        self.attack_vector = king.direct_attacks.clone();
        self.check = king.check;
    }

    pub fn finish_turn(&mut self) {
        self.board.update_controlled_squares(&self.current_player);
        self.turn += 1;
        self.current_player = self.current_player.other();
    }

    // This function allow to plug IA into the engine, they will be trusted and bypass move legality checking
    // If the IA does weird stuff, it could block the whole system
    pub fn play_bypass(&mut self, moves: Vec<Move>) {
        let curr_board = self.board.clone();
        self.board_history.push(curr_board);
        for m in moves {
            self.board.play_once(m);
        }
    }

    pub fn play_once(&mut self, m: Move) -> Option<Position> {
        let curr_board = self.board.clone();
        self.board_history.push(curr_board);
        self.board.play_once(m)
    }

    pub fn is_current_player_piece(&self, pos: &Position) -> bool {
        match self.board.board[pos.0 as usize][pos.1 as usize].color() {
            Some(c) => c == self.current_player,
            _ => false
        }
    }

    pub fn gen_all_moves(&self) -> Vec<Move> {
        self.board.gen_all_moves(&self.current_player, self.check, &self.attack_vector)
    }
}

pub struct PlayerVsPlayer {
    game_engine: GameEngine,
    current_selection: Option<Position>,
    can_promote: Option<Position>
}


pub struct PlayerVsIa {
    game_engine: GameEngine,
    player_color: Color,
    current_selection: Option<Position>,
    current_moves: HashMap<Position, Move>,
    can_promote: Option<Position>,
    ai: Box<dyn Ai>
}

impl PlayerVsIa {
    pub fn new(player_color: Color, ai: Box<dyn Ai>) -> Self {
        Self {
            game_engine: GameEngine::new(),
            player_color: player_color,
            current_selection: None,
            current_moves: HashMap::new(),
            can_promote: None,
            ai: ai
        }
    }
}

impl PlayerVsIa {
    pub fn ai_play(&mut self) {
        let ai_moves = self.ai.play(&self.game_engine);
        self.game_engine.play_bypass(ai_moves);
        self.game_engine.finish_turn();
        self.game_engine.prepare_new_turn();
    }
}

impl Game for PlayerVsIa {
    fn play(&mut self, m: Play) {
        // If its the Ai turn to play, we juste ignore the player input
        if self.game_engine.current_player != self.player_color {
            return
        }
        // If the player could promote but didn't, we have to rollback to his previous move
        if self.can_promote.is_some() {
            self.game_engine.rollback();
            self.can_promote = None;
        }
        // If the play is out of bounds, we reset (we do not need to check for negative since its usize)
        if m.x > 7 || m.y > 7 {
            self.current_moves.clear();
            self.current_selection = None;
        }
        let pos = (m.x as i8, m.y as i8);
        // If the player selects a move
        if let Some(m) =  self.current_moves.get(&pos) {
            self.can_promote = self.game_engine.play_once(m.clone());
            // We only finish the turn if the player can't promote
            if self.can_promote.is_none() {
                self.game_engine.finish_turn();
                self.game_engine.prepare_new_turn();
                // The ai plays after the player
                self.ai_play();
            }
            self.current_selection = None;
            self.current_moves.clear();
        }
        // If the player select one of his pieces
        else if self.game_engine.is_current_player_piece(&pos) {
            self.current_selection = Some(pos.clone());
            self.current_moves = self.game_engine.get_moves_for_piece(&pos);
        }
    }

    fn promote(&mut self, p: Promote) {
        if let Some(pr) = self.can_promote {
            let m = Move::Promote(pr, p.promote_to.into());
            self.game_engine.play_once(m);
            self.game_engine.finish_turn();
            self.game_engine.prepare_new_turn();
            self.can_promote = None;
            self.current_selection = None;
            self.current_moves.clear();
            // The ai plays after the player
            self.ai_play();
        }
    }

    fn webapp_repr(&self) -> GameWebappRepr {
        let mut board_repr = self.game_engine.to_webapp();
        if !self.current_moves.is_empty() {
            apply_markers(&mut board_repr, &self.current_moves.values().cloned().collect())
        }
        let board_history = self.game_engine.board_history.iter().map(|b| b.to_webapp()).collect();
        GameWebappRepr {
            current_player: self.game_engine.current_player.clone(),
            turn: self.game_engine.turn,
            board: board_repr,
            board_history: board_history,
        }
    }
}


#[derive(Serialize, Deserialize, Message)]
#[rtype(result="Result<GameWebappRepr, ()>")]
pub struct Play {
    x: usize,
    y: usize
}


#[derive(Serialize, Deserialize, Message)]
#[rtype(result="Result<GameWebappRepr, ()>")]
pub struct Promote {
    promote_to: CanPromoteTo
}

pub enum GameEvent {
    Play(Play),
    Promote(Promote)
}

pub struct IaVsIa {
    game_engine: GameEngine,
    min_ia_time_to_play: usize,
    max_ia_time_to_play: usize,
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GameWebappRepr {
    current_player: Color,
    turn: usize,
    board: WebappRepr,
    board_history: Vec<WebappRepr>
}

pub trait Game {
    fn play(&mut self, m: Play);

    fn promote(&mut self, p: Promote);

    fn webapp_repr(&self) -> GameWebappRepr;
}
