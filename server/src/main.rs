#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::State;
use rocket_contrib::json::{Json, JsonValue};

use serde::{Deserialize, Serialize};

use uuid::Uuid;

use common::Game;
use common::Lobby;
use common::tictactoe::PlayerAction;

use std::collections::HashMap;

struct AppState {
    lobbies: HashMap<String, Lobby>,
}

#[get("/lobbies")]
fn list_games(state: State<AppState>) -> Json<Vec<Lobby>> {
    unimplemented!()
}

/// Get the status of the game
///
///
#[get("/lobbies/:id")]
fn game_status() {}

#[derive(Serialize, Deserialize)]
struct JoinResponse {
    player_id: Uuid,
}

/// Join the game, get a player identifier UUID
#[post("/lobbies/<lobby>/join")]
fn join_game(lobby: String, state: State<AppState>) -> Json<JoinResponse> {
    
    unimplemented!()
    
    //Json(state.lobbies.clone())
}

/// Join the game, get a player identifier UUID
#[post("/lobbies/<lobby>/action", data = "<body>")]
fn perform_action(lobby: String, body: Json<PlayerAction>, state: State<AppState>) -> Json<Game> {
    unimplemented!()
}

/// Join the game, get a player identifier UUID
#[get("/lobbies/<lobby>/state")]
fn get_state(lobby: String, state: State<AppState>) -> Json<Game> {
    unimplemented!()
}

fn main() {
    let mut lobbies = HashMap::new();

    lobbies.insert(
        String::from("cool kids only"),
        Lobby {
            name: String::from("cool kids only"),
            players: 0,
            max_players: 1,
            game: Game::TicTacToe(common::tictactoe::GameState::default()),
        },
    );

    lobbies.insert(
        String::from("actually cool kids"),
        Lobby {
            name: String::from("actually cool kids"),
            players: 0,
            max_players: 1,
            game: Game::RockPaperScissors,
        },
    );

    rocket::ignite()
        .manage(AppState {
            lobbies
        })
        .mount("/", routes![list_games])
        .launch();
}
