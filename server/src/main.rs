#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::State;
use rocket_contrib::json::{Json, JsonValue};

use serde_json::json;

use uuid::Uuid;

use common::tictactoe::PlayerAction;
use common::CreateLobbyRequest;
use common::Game;
use common::Lobby;

use parking_lot::Mutex;
use std::collections::HashMap;

use common::JoinResponse;

struct AppState {
    lobbies: Mutex<HashMap<String, Lobby>>,
}

#[get("/lobbies")]
fn list_games(state: State<AppState>) -> Json<HashMap<String, Lobby>> {
    Json(state.lobbies.lock().clone())
}

/// Join the game, get a player identifier UUID
#[post("/lobbies/<lobby>/join")]
fn join_game(lobby: String, state: State<AppState>) -> JsonValue {
    let player = Uuid::new_v4();

    let val = perform_action(lobby.clone(), Json(PlayerAction::Join { player }), state);

    if val.get("error").is_some() {
        return val;
    } /*else {
          state.lobbies.lock().get_mut(&lobby).unwrap().players += 1;
      }*/

    JsonValue(serde_json::to_value(JoinResponse { player }).unwrap())
}

/// Join the game, get a player identifier UUID
#[post("/lobbies/<lobby>/action", data = "<body>")]
fn perform_action(lobby: String, body: Json<PlayerAction>, state: State<AppState>) -> JsonValue {
    let res = match state.lobbies.lock().get_mut(&lobby) {
        Some(lobby) => {
            let new_state = match lobby.game.clone() {
                Game::TicTacToe(state) => match common::tictactoe::process_input(body.0, state) {
                    Ok(new_state) => Game::TicTacToe(new_state),
                    Err(e) => {
                        return JsonValue(json!({
                            "error": serde_json::to_value(e).unwrap()
                        }))
                    }
                },
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

/// Get the status of the game
///
///
#[post("/lobbies", data = "<lobby>")]
fn create_lobby(lobby: Json<CreateLobbyRequest>, state: State<AppState>) -> JsonValue {
    state.lobbies.lock().insert(
        lobby.0.name.clone(),
        Lobby {
            name: lobby.0.name.clone(),
            players: 0,
            max_players: 2,
            game: Game::from(lobby.0.game),
        },
    );

    JsonValue(json!({}))
}

fn main() {
    rocket::ignite()
        .manage(AppState {
            lobbies: Mutex::new(HashMap::new()),
        })
        .mount(
            "/",
            routes![
                list_games,
                join_game,
                get_state,
                perform_action,
                create_lobby
            ],
        )
        .launch();
}
