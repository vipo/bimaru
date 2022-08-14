mod setups;

use serde::{Deserialize, Serialize};
use std::str::FromStr;
use setups::{build_all, Setups, OccupiedCells};
use tide::{Request, Response, Server};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
#[warn(dead_code)]
struct NewGame {
    #[serde(with = "uuid_as_string")]
    id: Uuid,
    #[serde(with = "uuid_as_string")]
    game_setup_id: Uuid,
    number_of_hints: u8,
    occupied_rows: [u8; 10],
    occupied_cols: [u8; 10],
}

#[derive(Clone)]
struct State {
    setups: Setups,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let state: State = State {
        setups: build_all(),
    };
    let mut app: Server<State> = tide::with_state(state);
    app.at("/v1/game/:setup_id").post(new_game_v1);
    app.listen("0.0.0.0:8080").await?;
    Ok(())
}

async fn new_game_v1(req: Request<State>) -> tide::Result {
    let game_setup_str: &str = req.param("setup_id")?;
    if let Ok(game_setup_id) = Uuid::from_str(game_setup_str) {
        if let Some(t) = req.state().setups.get(&game_setup_id) {
            let response_entity: NewGame = NewGame {
                id: Uuid::new_v4(),
                game_setup_id,
                number_of_hints: 10,
                occupied_cols: t.occupied_cols(),
                occupied_rows: t.occupied_rows(),
            };
            yaml_response(201, &response_entity)
        } else {
            not_found("Game template not found")
        }
    } else {
        not_found("Template id not found")
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

fn not_found(text: &str) -> tide::Result {
    Ok(Response::builder(404).body(text).build())
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
