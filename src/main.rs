mod templates;

use serde::{Serialize, Deserialize};
use std::str::FromStr;
use templates::{build_all, Templates};
use tide::{Request, Response, Server};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
#[warn(dead_code)]
struct NewGame {
    #[serde(with = "uuid_as_string")]
    id: Uuid,
    #[serde(with = "uuid_as_string")]
    game_template_id: Uuid,
    number_of_hints: u8,
}

/// The shared application state.
#[derive(Clone)]
struct State {
    templates: Templates,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let state: State = State {
        templates: build_all(),
    };
    let mut app: Server<State> = tide::with_state(state);
    app.at("/game/:template_id").post(new_game);
    app.listen("0.0.0.0:8080").await?;
    Ok(())
}

async fn new_game(req: Request<State>) -> tide::Result {
    let game_template_str: &str = req.param("template_id")?;
    let game_template_id: Uuid = Uuid::from_str(game_template_str)?;
    match req.state().templates.get(&game_template_id) {
        None => Ok(Response::builder(404)
            .body("Game template not found")
            .build()),
        Some(_) => {
            let response_entity: NewGame = NewGame {
                id: Uuid::new_v4(),
                game_template_id: game_template_id,
                number_of_hints: 20,
            };
            let response: Response = yaml_response(201, &response_entity)?;
            Ok(response)
        }
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
