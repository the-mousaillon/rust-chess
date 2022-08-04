use std::collections::HashMap;

use actix::prelude::*;
use actix_web::web;
use serde::{Serialize, Deserialize};

use crate::{piece::{Color, Position, Piece, Move, PieceType, CanPromoteTo, King}, chessbord::{WebappRepr, ChessBoard, apply_markers}, game::{GameEngine, Game, Play, Promote, PlayerVsIa, GameWebappRepr, AiVsAi}, ai::{DummyRandomIA, Ai, BestPlayDephtOneAi, MiniMaxAi}};

struct ChessActor {
    game: Option<Box<dyn Game>>
}

impl ChessActor {
    pub fn new() -> Self {
        Self {
            game: None
        }
    }
}

impl Actor for ChessActor {
    type Context = Context<Self>;
}


impl Handler<Play> for ChessActor {
    type Result=Result<GameWebappRepr, ()>;

    fn handle(&mut self, msg: Play, ctx: &mut Self::Context) -> Self::Result {
        self.game.as_mut().ok_or(()).map(|g| {
            g.play(msg);
            g.webapp_repr()
        })
    }
}

impl Handler<BoardActions> for ChessActor {
    type Result=Result<GameWebappRepr, ()>;

    fn handle(&mut self, msg: BoardActions, ctx: &mut Self::Context) -> Self::Result {
        match msg {
            BoardActions::Setup(mode) => {
                match mode {
                    GameMode::PlayerVsPlayer => todo!(),
                    GameMode::PlayerVsAi(player_color, ai_implementation) => {
                        let ai= ai_implementation.instantiate(&player_color.other());
                        let game = PlayerVsIa::new(player_color, ai);
                        self.game = Some(Box::new(game));
                        Ok(self.game.as_ref().unwrap().webapp_repr())
                    },
                    GameMode::AiVsAi(ai_implementation) => {
                        let black_ai = ai_implementation.instantiate(&Color::Black);
                        let white_ai = ai_implementation.instantiate(&Color::White);
                        let game = AiVsAi::new(white_ai, black_ai);
                        self.game = Some(Box::new(game));
                        Ok(self.game.as_ref().unwrap().webapp_repr())
                    },
                }
            },
            _ => {
                Err(())
            }
        }
    }
}


impl Handler<Promote> for ChessActor {
    type Result=Result<GameWebappRepr, ()>;

    fn handle(&mut self, msg: Promote, ctx: &mut Self::Context) -> Self::Result {
        self.game.as_mut().ok_or(()).map(|g| {
            g.promote(msg);
            g.webapp_repr()
        })
    }
}


struct AppData {
    chess_actor: Addr<ChessActor>
}


async fn reset_board(data: web::Data<AppData>, action: web::Json<BoardActions>) -> actix_web::Result<impl actix_web::Responder> {
    let err = actix_web::error::ErrorInternalServerError("could not process play");
    let new_board = data.chess_actor.send(action.0).await.unwrap().map_err(|e| err)?;
    Ok(web::Json(new_board))
}

#[derive(Serialize, Deserialize, Message)]
#[rtype(result="Result<GameWebappRepr, ()>")]
pub enum AiImplementation {
    DummyAi,
    BestPlayDephtOneAi,
    MiniMaxAi
}

impl AiImplementation {
    pub fn instantiate(&self, color: &Color) -> Box<dyn Ai> {
        match self {
            AiImplementation::DummyAi => Box::new(DummyRandomIA::new(color.other())) as Box<dyn Ai>,
            AiImplementation::BestPlayDephtOneAi => Box::new(BestPlayDephtOneAi::new(color.other())) as Box<dyn Ai>,
            AiImplementation::MiniMaxAi => Box::new(MiniMaxAi::new(color.other())) as Box<dyn Ai>,
        }
    }
}


#[derive(Serialize, Deserialize, Message)]
#[rtype(result="Result<GameWebappRepr, ()>")]
pub enum GameMode {
    PlayerVsPlayer,
    PlayerVsAi(Color, AiImplementation),
    AiVsAi(AiImplementation)
}


#[derive(Serialize, Deserialize, Message)]
#[rtype(result="Result<GameWebappRepr, ()>")]
enum BoardActions {
    Setup(GameMode),
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


async fn promote(data: web::Data<AppData>, payload: web::Json<Promote>) -> actix_web::Result<impl actix_web::Responder> {
    let err = actix_web::error::ErrorInternalServerError("Bod promotion");
    let new_board = data.chess_actor.send(payload.0).await.unwrap().map_err(|_| err)?;
    Ok(web::Json(new_board))
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
            .route("/api/set_play_mode", web::post().to(reset_board))
            .route("/api/promote", web::post().to(promote))
            .wrap(cors)
    })
    .bind(("127.0.0.1", 8005))?
    .run()
    .await    
}

#[test]
fn test_ser() {
    let game_setup = BoardActions::Setup(GameMode::PlayerVsAi(Color::White, AiImplementation::BestPlayDephtOneAi));
    let ev = serde_json::to_string(&game_setup).unwrap();
    println!("ev: {}", ev);

    //let promote_str = "{}"
}