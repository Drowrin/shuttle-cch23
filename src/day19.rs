use std::{collections::HashMap, sync::Arc};

use axum::{
    extract::{
        ws::{Message, WebSocket},
        Path, State, WebSocketUpgrade,
    },
    response::Response,
    routing, Router,
};
use futures::{stream::SplitSink, SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tokio::sync::Mutex;

async fn ping(wsu: WebSocketUpgrade) -> Response {
    wsu.on_upgrade(|mut ws| async move {
        let mut started = false;

        while let Some(msg) = ws.recv().await {
            if let Ok(msg) = msg {
                match msg.to_text().unwrap() {
                    "serve" => started = true,
                    "ping" if started => ws.send("pong".into()).await.unwrap(),
                    _ => (),
                }
            } else {
                return;
            }
        }
    })
}

#[derive(Default, Clone)]
struct AppState {
    rooms: Arc<Mutex<HashMap<usize, HashMap<String, Arc<Mutex<SplitSink<WebSocket, Message>>>>>>>,
    views: Arc<Mutex<usize>>,
}

#[derive(Serialize, Deserialize)]
struct Tweet {
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    user: Option<String>,
}

async fn reset(State(state): State<AppState>) {
    *state.views.lock().await = 0;
}

async fn views(State(state): State<AppState>) -> String {
    state.views.lock().await.to_string()
}

async fn room(
    wsu: WebSocketUpgrade,
    State(state): State<AppState>,
    Path((number, user)): Path<(usize, String)>,
) -> Response {
    wsu.on_upgrade(move |ws| async move {
        let (sender, receiver) = ws.split();

        {
            state
                .rooms
                .lock()
                .await
                .entry(number)
                .or_default()
                .insert(user.clone(), Arc::new(Mutex::new(sender)));
        }

        receiver
            .for_each(|msg| async {
                if let Ok(msg) = msg {
                    if let Ok(mut m) = serde_json::from_slice::<Tweet>(&msg.into_data()) {
                        if m.message.len() > 128 {
                            return;
                        }

                        m.user = Some(user.clone());
                        let binding = state.rooms.lock().await;
                        let room = binding.get(&number).unwrap();

                        for u_ws in room.values() {
                            u_ws.lock()
                                .await
                                .send(serde_json::to_string(&m).unwrap().into())
                                .await
                                .unwrap();
                        }

                        *state.views.lock().await += room.len();
                    }
                }
            })
            .await;

        state
            .rooms
            .lock()
            .await
            .get_mut(&number)
            .unwrap()
            .remove(&user);
    })
}

pub fn get_routes() -> Router<PgPool> {
    Router::new()
        .route("/19/ws/ping", routing::get(ping))
        .route("/19/reset", routing::post(reset))
        .route("/19/views", routing::get(views))
        .route("/19/ws/room/:number/user/:string", routing::get(room))
        .with_state(AppState::default())
}
