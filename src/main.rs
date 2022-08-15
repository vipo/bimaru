mod setups;

use serde::{Deserialize, Serialize};
use setups::{build_all, OccupiedCells, Setup, Setups};
use std::sync::{Arc, RwLock};
use std::{collections::HashMap, str::FromStr};
use tide::{Request, Response, Server};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
struct NewGame {
    #[serde(with = "uuid_as_string")]
    session_id: Uuid,
    #[serde(with = "uuid_as_string")]
    game_setup_id: Uuid,
    number_of_hints: u8,
    occupied_rows: [u8; 10],
    occupied_cols: [u8; 10],
}

pub trait IsSolved {
    fn solves(&self, setup: Setup) -> bool;
}

#[derive(Debug, Serialize, Deserialize)]
struct CheckV1 {
    coords: Vec<CoordV1>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct CoordV1 {
    col: usize,
    row: usize,
}

impl IsSolved for CheckV1 {
    fn solves(&self, setup: Setup) -> bool {
        if self.coords.len() == 20 {
            for i in setups::MIN_INDEX..=setups::MAX_INDEX {
                for j in setups::MIN_INDEX..=setups::MAX_INDEX {
                    if setup[i][j] > 0 && !self.coords.contains(&CoordV1 { col: j, row: i }) {
                        return false;
                    }
                }
            }
            true
        } else {
            false
        }
    }
}

#[derive(Clone)]
struct State {
    setups: Arc<Setups>,
    sessions: Arc<RwLock<HashMap<Uuid, u8>>>,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let app: Server<State> = build_app();
    app.listen("0.0.0.0:8080").await?;
    Ok(())
}

fn build_app() -> Server<State> {
    let state: State = State {
        setups: Arc::new(build_all()),
        sessions: Arc::new(RwLock::new(HashMap::new())),
    };
    let mut app: Server<State> = tide::with_state(state);
    app.at("/v1/game/:setup_id").post(new_game_v1);
    app.at("/v1/game/:setup_id/session/:session_id/check")
        .post(check_v1);
    app
}

async fn check_v1(mut req: Request<State>) -> tide::Result {
    let game_setup_str: &str = req.param("setup_id")?;
    if let Ok(game_setup_id) = Uuid::from_str(game_setup_str) {
        if let Some(setup) = req.state().clone().setups.get(&game_setup_id) {
            let session_id_str: &str = req.param("session_id")?;
            if let Ok(session_id) = Uuid::from_str(session_id_str) {
                if req
                    .state()
                    .sessions
                    .read()
                    .unwrap()
                    .contains_key(&session_id)
                {
                    if let Ok(body_str) = req.body_string().await {
                        if let Ok(entity) = serde_yaml::from_str::<CheckV1>(&body_str) {
                            if entity.solves(*setup) {
                                finish()
                            } else {
                                try_harder()
                            }
                        } else {
                            illegal_request("Could not parse entity")
                        }
                    } else {
                        illegal_request("Could not read the request")
                    }
                } else {
                    not_found("Unknown session")
                }
            } else {
                not_found("Session id not found")
            }
        } else {
            not_found("Unknown game setup")
        }
    } else {
        not_found("Game setup id not found")
    }
}

async fn new_game_v1(req: Request<State>) -> tide::Result {
    let game_setup_str: &str = req.param("setup_id")?;
    if let Ok(game_setup_id) = Uuid::from_str(game_setup_str) {
        if let Some(s) = req.state().setups.get(&game_setup_id) {
            let session_id = Uuid::new_v4();
            req.state().sessions.write().unwrap().insert(session_id, 0);
            let response_entity: NewGame = NewGame {
                session_id,
                game_setup_id,
                number_of_hints: 10,
                occupied_cols: s.occupied_cols(),
                occupied_rows: s.occupied_rows(),
            };
            yaml_response(201, &response_entity)
        } else {
            not_found("Game template not found")
        }
    } else {
        not_found("Game setup id not found")
    }
}

fn yaml_response<T>(status: u16, value: &T) -> tide::Result
where
    T: Serialize,
{
    let yaml: String = serde_yaml::to_string(&value)?;
    Ok(Response::builder(status)
        .body(yaml)
        .content_type("text/x-yaml")
        .build())
}

fn illegal_request(text: &str) -> tide::Result {
    Ok(Response::builder(415)
        .body(text)
        .content_type("text/plain")
        .build())
}

fn not_found(text: &str) -> tide::Result {
    Ok(Response::builder(404)
        .body(text)
        .content_type("text/plain")
        .build())
}

fn finish() -> tide::Result {
    Ok(Response::builder(200)
        .body("Well done!")
        .content_type("text/plain")
        .build())
}

fn try_harder() -> tide::Result {
    Ok(Response::builder(409)
        .body("Try harder!")
        .content_type("text/plain")
        .build())
}

mod uuid_as_string {
    use serde::de::Error;
    use std::str::FromStr;
    use uuid::Uuid;

    pub fn serialize<S>(u: &Uuid, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serde::Serialize::serialize(&u.to_string(), serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Uuid, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Uuid::from_str(s).map_err(D::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tide_testing::TideTestingExt;

    #[async_std::test]
    async fn test_create_and_check_v1() {
        let game_setup_id = uuid::uuid!("dd8fb490-72c8-485b-aeea-537b9be34e4b");
        let app = build_app();
        let create_resp = app
            .post(format!("/v1/game/{}", game_setup_id))
            .recv_string()
            .await
            .unwrap();
        let create_entity = serde_yaml::from_str::<NewGame>(&create_resp).unwrap();
        assert_eq!(create_entity.number_of_hints, 10);
        assert_eq!(
            create_entity.occupied_rows.len() + create_entity.occupied_cols.len(),
            20
        );

        let check_v1_entity = CheckV1 {
            coords: vec![CoordV1 { col: 0, row: 0 }, CoordV1 { col: 1, row: 1 }],
        };
        let check_v1_str = serde_yaml::to_string(&check_v1_entity).unwrap();
        let check_v1_resp = app
            .post(format!(
                "/v1/game/{}/session/{}/check",
                game_setup_id, create_entity.session_id
            ))
            .body_string(check_v1_str)
            .await
            .unwrap();
        assert_eq!(check_v1_resp.status(), tide::http::StatusCode::Conflict);

        let check_v1_entity = CheckV1 {
            coords: vec![
                CoordV1 { col: 0, row: 1 },
                CoordV1 { col: 0, row: 4 },
                CoordV1 { col: 2, row: 3 },
                CoordV1 { col: 3, row: 6 },
                CoordV1 { col: 3, row: 7 },
                CoordV1 { col: 3, row: 8 },
                CoordV1 { col: 5, row: 0 },
                CoordV1 { col: 5, row: 2 },
                CoordV1 { col: 5, row: 3 },
                CoordV1 { col: 5, row: 4 },
                CoordV1 { col: 5, row: 6 },
                CoordV1 { col: 6, row: 0 },
                CoordV1 { col: 7, row: 0 },
                CoordV1 { col: 7, row: 7 },
                CoordV1 { col: 7, row: 9 },
                CoordV1 { col: 8, row: 0 },
                CoordV1 { col: 8, row: 7 },
                CoordV1 { col: 8, row: 9 },
                CoordV1 { col: 9, row: 4 },
                CoordV1 { col: 9, row: 5 },
            ],
        };
        let check_v1_str = serde_yaml::to_string(&check_v1_entity).unwrap();
        let check_v1_resp = app
            .post(format!(
                "/v1/game/{}/session/{}/check",
                game_setup_id, create_entity.session_id
            ))
            .body_string(check_v1_str)
            .await
            .unwrap();
        assert_eq!(check_v1_resp.status(), tide::http::StatusCode::Ok);
    }
}
