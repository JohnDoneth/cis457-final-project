#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]

#[macro_use]
extern crate rocket;

use rocket_contrib::json::{Json, JsonValue};
use rocket::State;

use serde::{Serialize, Deserialize};

use common::Lobby;

struct AppState {
    lobbies: Vec<Lobby>,
}

#[get("/lobbies")]
fn list_games(state: State<AppState>) -> Json<Vec<Lobby>> {
    Json(state.lobbies.clone())
}

fn main() {
    let state = AppState {
        lobbies: vec![Lobby {
            name: String::from("cool kids only"),
            players: 0,
            max_players: 1,
            game: String::from("Tic Tac Toe"),
        },
        Lobby {
            name: String::from("actually cool kids"),
            players: 0,
            max_players: 1,
            game: String::from("EXTREME Tic Tac Toe"),
        }],
    };

    rocket::ignite()
        .manage(state)
        .mount("/", routes![list_games])
        .launch();
}
