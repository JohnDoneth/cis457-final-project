#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::State;
use rocket_contrib::json::{Json, JsonValue};

use serde::{Deserialize, Serialize};

use serde_json::json;

use uuid::Uuid;

use common::tictactoe::PlayerAction;
use common::Game;
use common::Lobby;

use parking_lot::Mutex;
use std::collections::HashMap;

struct AppState {
    lobbies: Mutex<HashMap<String, Lobby>>,
}

#[get("/lobbies")]
fn list_games(state: State<AppState>) -> Json<HashMap<String, Lobby>> {
    Json(state.lobbies.lock().clone())
}

#[derive(Serialize, Deserialize)]
struct JoinResponse {
    player_id: Uuid,
}

/// Join the game, get a player identifier UUID
#[post("/lobbies/<lobby>/join")]
fn join_game(lobby: String, state: State<AppState>) -> Json<JoinResponse> {
    Json(JoinResponse {
        player_id: Uuid::new_v4(),
    })
}

/// Join the game, get a player identifier UUID
#[post("/lobbies/<lobby>/action", data = "<body>")]
fn perform_action(lobby: String, body: Json<PlayerAction>, state: State<AppState>) -> JsonValue {
    let res = match state.lobbies.lock().get_mut(&lobby) {
        Some(lobby) => {
            let new_state = match lobby.game.clone() {
                Game::TicTacToe(state) => {
                    Game::TicTacToe(common::tictactoe::process_input(body.0, state))
                }
                Game::RockPaperScissors => unimplemented!(),
            };

            lobby.game = new_state;

            serde_json::to_value(lobby.game.clone()).unwrap()
        }
        None => json!({
            "error": "lobby not found"
        }),
    };

    JsonValue(res)
}

/// Get the status of the game
///
///
#[get("/lobbies/<lobby>/state")]
fn get_state(lobby: String, state: State<AppState>) -> JsonValue {
    let res: serde_json::Value = match state.lobbies.lock().get(&lobby) {
        Some(lobby) => serde_json::to_value(lobby.game.clone()).unwrap(),
        None => json!({
            "error": "lobby not found"
        }),
    };

    JsonValue(res)
}

fn main() {
    println!(
        "{}",
        serde_json::to_string_pretty(&PlayerAction::PlaceToken {
            player: Uuid::new_v4(),
            position: (0, 0)
        })
        .unwrap()
    );

    let mut lobbies = HashMap::new();

    lobbies.insert(
        String::from("lobby1"),
        Lobby {
            name: String::from("lobby1"),
            players: 0,
            max_players: 1,
            game: Game::TicTacToe(common::tictactoe::GameState::default()),
        },
    );

    lobbies.insert(
        String::from("lobby2"),
        Lobby {
            name: String::from("lobby2"),
            players: 0,
            max_players: 1,
            game: Game::RockPaperScissors,
        },
    );

    rocket::ignite()
        .manage(AppState {
            lobbies: Mutex::new(lobbies),
        })
        .mount(
            "/",
            routes![list_games, join_game, get_state, perform_action],
        )
        .launch();
}
