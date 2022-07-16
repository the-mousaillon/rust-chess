use std::collections::HashMap;

use actix::prelude::*;
use actix_web::web;
use serde::{Serialize, Deserialize};

use crate::{piece::{Color, Position, Piece, Move}, chessbord::{WebappRepr, ChessBoard, apply_markers}};

struct ChessActor {
    board: ChessBoard,
    current_player: Color,
    turn: usize,
    current_selection: Option<Piece>,
    current_moves: HashMap<Position, Move>
}

impl ChessActor {
    pub fn new() -> Self {
        Self {
            board: ChessBoard::new_default(),
            current_player: Color::White,
            turn: 0,
            current_selection: None,
            current_moves: HashMap::new()
        }
    }
}

impl Actor for ChessActor {
    type Context = Context<Self>;
}


impl Handler<Play> for ChessActor {
    type Result=Result<WebappRepr, ()>;

    fn handle(&mut self, msg: Play, ctx: &mut Self::Context) -> Self::Result {
        if msg.x > 7 || msg.y > 7 {
            return Err(())
        }
        let dst = &self.board.board[msg.x][msg.y];
        println!("dst actix: {:?}", dst);
        let is_empty = dst.is_empty();
        let is_same_color = dst.color().map(|c| c == self.current_player).unwrap_or(false);
        match (is_empty, is_same_color, &self.current_selection) {
            // The player plays somewhere
            (_, false, Some(p)) => {
                let play = self.current_moves.get(&(msg.x as i8, msg.y as i8));
                println!("PLAY: {:?}", play);
                match play {
                    // The play is valid
                    Some(m) => {
                        self.board.play(vec![m.clone()]);
                        self.turn += 1;
                        match self.current_player {
                            Color::White => self.current_player = Color::Black,
                            Color::Black => self.current_player = Color::White,
                        }
                    },
                    // The play is invalid
                    _ => {}
                }
                self.current_selection = None;
                self.current_moves.clear();

            },
            // The player selects a new piece (or casle be we will see that later)
            (false, true, Some(p)) => {
                self.current_selection = Some(dst.clone());
                self.current_moves = HashMap::from_iter(
                    dst.gen_moves(&self.board)
                       .into_iter()
                       .map(|m| (m.to(), m))
                       .filter(|(to, _)| to.is_some())
                       .map(|(to, m)| (to.unwrap(), m))
                )
            },
            // The player makes a selection
            (false, true, None) => {
                self.current_selection = Some(dst.clone());
                self.current_moves = HashMap::from_iter(
                    dst.gen_moves(&self.board)
                       .into_iter()
                       .map(|m| (m.to(), m))
                       .filter(|(to, _)| to.is_some())
                       .map(|(to, m)| (to.unwrap(), m))
                )
            },
            // The player clicks on opposite piece without selection
            (false, false, None) => {
                self.current_moves.clear();
                self.current_selection = None;
                return Err(())
            },
            _ => return Err(())
        };
        self.board.pprint();
        let attack_vectors = self.board.locate_king(Color::White).gen_attack_vectors(&self.board);
        println!("ATTACK VECTORS: {:?}", attack_vectors);
        println!("CURRENT MOVES: {:?}", self.current_moves);
        println!("CURRENT SELECTION: {:?}", self.current_selection);
        let mut board_repr = self.board.to_webapp();
        if !self.current_moves.is_empty() {
            apply_markers(&mut board_repr, &self.current_moves.values().cloned().collect())
        }
        Ok(board_repr)
    }
}

impl Handler<BoardActions> for ChessActor {
    type Result=Result<WebappRepr, ()>;

    fn handle(&mut self, msg: BoardActions, ctx: &mut Self::Context) -> Self::Result {
        match msg {
            BoardActions::Setup => {
                self.board = ChessBoard::new_default();
                self.current_player = Color::White;
                self.turn = 0;
                self.current_selection = None;
                self.current_moves = HashMap::new();
                Ok(self.board.to_webapp())
            },
            _ => {
                Err(())
            }
        }
    }
}

struct AppData {
    chess_actor: Addr<ChessActor>
}


async fn reset_board(data: web::Data<AppData>) -> actix_web::Result<impl actix_web::Responder> {
    let err = actix_web::error::ErrorInternalServerError("could not process play");
    let new_board = data.chess_actor.send(BoardActions::Setup).await.unwrap().map_err(|e| err)?;
    Ok(web::Json(new_board))
}

#[derive(Serialize, Deserialize, Message)]
#[rtype(result="Result<WebappRepr, ()>")]
struct Play {
    x: usize,
    y: usize
}

#[derive(Serialize, Deserialize, Message)]
#[rtype(result="Result<WebappRepr, ()>")]
enum BoardActions {
    Setup
}

async fn play(data: web::Data<AppData>, payload: web::Json<Play>) -> actix_web::Result<impl actix_web::Responder> {
    let err = actix_web::error::ErrorInternalServerError("could not process play");
    let new_board = data.chess_actor.send(payload.0).await.unwrap().map_err(|_| err)?;
    Ok(web::Json(new_board))
}

async fn set_play_mode(data: web::Data<AppData>, mode: String) -> actix_web::Result<impl actix_web::Responder> {
    let err = actix_web::error::ErrorInternalServerError("could not process play");
   // let new_board = data.chess_actor.send(payload.0).await.unwrap().map_err(|_| err)?;
   // Ok(web::Json(new_board))
    if false {
        return Ok(web::Json("lol"))
    }
    Err(err)
}

//#[actix::main]
pub async fn run_dev_app() -> std::io::Result<()> {
    let chess_actor = ChessActor::new();
    let chess_actor = chess_actor.start();
    actix_web::HttpServer::new(move || {
        let cors = actix_cors::Cors::default()
        .allow_any_header()
        .allow_any_method()
        .allow_any_origin();

        actix_web::App::new()
            .app_data(web::Data::new(AppData {
                chess_actor: chess_actor.clone()
            }))
            .route("/api/play", web::post().to(play))
            .route("/api/reset_board", web::get().to(reset_board))
            .route("/api/set_play_mode", web::get().to(reset_board))
            .wrap(cors)
    })
    .bind(("127.0.0.1", 8005))?
    .run()
    .await    
}