mod setups;

use serde::{Deserialize, Serialize};
use setups::{build_all, CreateFormat, HintFormat, OccupiedCells, Searchable, Setup, Setups};
use std::str::FromStr;
use std::sync::Arc;
use tide::{Request, Response, Server};
use uuid::Uuid;
use string_builder::Builder;

const MAX_HINTS: u8 = 10;

#[derive(Serialize, Deserialize)]
struct NewGame {
    #[serde(with = "uuid_as_string")]
    game_setup_id: Uuid,
    number_of_hints: u8,
    occupied_rows: [u8; 10],
    occupied_cols: [u8; 10],
}

#[derive(Serialize, Deserialize)]
struct NestedNewGame {
    #[serde(with = "uuid_as_string")]
    game_setup_id: Uuid,
    number_of_hints: u8,
    occupied_rows: Option<NonEmptyList<u8>>,
    occupied_cols: Option<NonEmptyList<u8>>,
}

pub trait IsSolved {
    fn solves(&self, setup: Setup) -> bool;
}

#[derive(Debug, Serialize, Deserialize)]
struct Check {
    coords: Vec<Coord>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Hints {
    coords: Vec<Coord>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct NestedHints {
    coords: Option<NonEmptyList<Coord>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
struct Coord {
    col: usize,
    row: usize,
}

impl IsSolved for Check {
    fn solves(&self, setup: Setup) -> bool {
        if self.coords.len() == 20 {
            for i in setups::MIN_INDEX..=setups::MAX_INDEX {
                for j in setups::MIN_INDEX..=setups::MAX_INDEX {
                    if setup[i][j] > 0 && !self.coords.contains(&Coord { col: j, row: i }) {
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

fn find_hints(setup: &Setup, limit: u8) -> Vec<Coord> {
    let mut result = Vec::with_capacity(limit as usize);
    for n in 1..=limit {
        if let Some(t) = setup.find_position(n) {
            result.push(Coord { row: t.0, col: t.1 });
        }
    }
    result
}

fn render_board(setup: &Setup) -> String {
    let mut builder = Builder::default();
    builder.append(" 0123456789");
    for i in setups::MIN_INDEX..=setups::MAX_INDEX {
        builder.append('\n');
        builder.append(i.to_string());
        for j in setups::MIN_INDEX..=setups::MAX_INDEX {
            if setup[i][j] > 0 {
                builder.append('#')
            } else {
                builder.append(' ')
            }
        }
    }
    builder.string().unwrap()
}

#[derive(Clone)]
struct State {
    setups: Arc<Setups>,
}
#[derive(Deserialize)]
struct HintQuery {
    limit: u8,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct NonEmptyList<T> {
    pub head: T,
    pub tail: Option<Box<NonEmptyList<T>>>,
}

fn to_non_empty_list<T>(v: &mut Vec<T>) -> Option<NonEmptyList<T>>
where
    T: Copy + Clone,
{
    if v.is_empty() {
        None
    } else {
        v.reverse();
        let mut tail = NonEmptyList {
            head: *v.first().unwrap(),
            tail: None,
        };
        for i in 1..v.len() {
            tail = NonEmptyList {
                head: *v.get(i).unwrap(),
                tail: Some(Box::new(tail)),
            };
        }
        Some(tail)
    }
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
    };
    let mut app: Server<State> = tide::with_state(state);
    app.at("/game/:setup_id").post(new_game);
    app.at("/game/:setup_id/check").post(check);
    app.at("/game/:setup_id/hint").get(make_hint);
    app.at("/game/:setup_id/board").get(show_board);

    app
}

async fn show_board(req: Request<State>) -> tide::Result {
    let game_setup_str: &str = req.param("setup_id")?;
    if let Ok(game_setup_id) = Uuid::from_str(game_setup_str) {
        if let Some(setup) = req.state().clone().setups.get(&game_setup_id) {
            text_response(&render_board(&setup.setup))
        } else {
            not_found("Unknown game setup")
        }
    } else {
        not_found("Game setup id not found")
    }
}

async fn make_hint(req: Request<State>) -> tide::Result {
    let game_setup_str: &str = req.param("setup_id")?;
    if let Ok(game_setup_id) = Uuid::from_str(game_setup_str) {
        if let Some(setup) = req.state().clone().setups.get(&game_setup_id) {
            let limit: u8 = match req.query::<HintQuery>() {
                Ok(v) => v.limit,
                Err(_) => 0,
            };
            let mut hints: Vec<Coord> = find_hints(&setup.setup, limit.min(MAX_HINTS));
            match setup.hint_format {
                HintFormat::Nested => yaml_response(
                    200,
                    CT_YAML_NESTED,
                    &(NestedHints {
                        coords: to_non_empty_list(&mut hints),
                    }),
                ),
                HintFormat::List => yaml_response(200, CT_YAML, &(Hints { coords: hints })),
            }
        } else {
            not_found("Unknown game setup")
        }
    } else {
        not_found("Game setup id not found")
    }
}

async fn check(mut req: Request<State>) -> tide::Result {
    let game_setup_str: &str = req.param("setup_id")?;
    if let Ok(game_setup_id) = Uuid::from_str(game_setup_str) {
        if let Some(setup) = req.state().clone().setups.get(&game_setup_id) {
            if let Ok(body_str) = req.body_string().await {
                if let Ok(entity) = serde_yaml::from_str::<Check>(&body_str) {
                    if entity.solves(setup.setup) {
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
            not_found("Unknown game setup")
        }
    } else {
        not_found("Game setup id not found")
    }
}

async fn new_game(req: Request<State>) -> tide::Result {
    let game_setup_str: &str = req.param("setup_id")?;
    if let Ok(game_setup_id) = Uuid::from_str(game_setup_str) {
        if let Some(sf) = req.state().setups.get(&game_setup_id) {
            let s = sf.setup;
            match sf.create_format {
                CreateFormat::Nested => {
                    let resp = NestedNewGame {
                        game_setup_id,
                        number_of_hints: MAX_HINTS,
                        occupied_cols: to_non_empty_list(&mut s.occupied_cols().to_vec()),
                        occupied_rows: to_non_empty_list(&mut s.occupied_rows().to_vec()),
                    };
                    yaml_response(201, CT_YAML_NESTED, &resp)
                }
                CreateFormat::List => {
                    let resp = NewGame {
                        game_setup_id,
                        number_of_hints: MAX_HINTS,
                        occupied_cols: s.occupied_cols(),
                        occupied_rows: s.occupied_rows(),
                    };
                    yaml_response(201, CT_YAML, &resp)
                }
            }
        } else {
            not_found("Game template not found")
        }
    } else {
        not_found("Game setup id not found")
    }
}

const CT_YAML: &str = "text/x-yaml";
const CT_YAML_NESTED: &str = "text/x-yaml-nested-lists";
const CT_PLAIN: &str = "text/plain";

fn yaml_response<T>(status: u16, ct: &str, value: &T) -> tide::Result
where
    T: Serialize,
{
    let yaml: String = serde_yaml::to_string(&value)?;
    Ok(Response::builder(status)
        .body(yaml)
        .content_type(ct)
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
        .content_type(CT_PLAIN)
        .build())
}

fn text_response(text: &str) -> tide::Result {
    Ok(Response::builder(200)
        .body(text)
        .content_type(CT_PLAIN)
        .build())
}

fn finish() -> tide::Result {
    text_response("Well done!")
}

fn try_harder() -> tide::Result {
    text_response("Try harder!")
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

    #[test]
    fn test_to_non_empty() {
        assert_eq!(to_non_empty_list::<u8>(&mut vec![]), None);
        assert_eq!(
            to_non_empty_list(&mut vec!['a']),
            Some(NonEmptyList {
                head: 'a',
                tail: None
            })
        );
        assert_eq!(
            to_non_empty_list(&mut vec![1, 2, 3]),
            Some(NonEmptyList {
                head: 1,
                tail: Some(Box::new(NonEmptyList {
                    head: 2,
                    tail: Some(Box::new(NonEmptyList {
                        head: 3,
                        tail: None
                    }))
                }))
            })
        );
    }

    #[async_std::test]
    async fn test_create_and_check() {
        let game_setup_id = uuid::uuid!("dd8fb490-72c8-485b-aeea-537b9be34e4b");
        let app = build_app();
        let create_resp = app
            .post(format!("/game/{}", game_setup_id))
            .recv_string()
            .await
            .unwrap();
        let create_entity = serde_yaml::from_str::<NewGame>(&create_resp).unwrap();
        assert_eq!(create_entity.number_of_hints, 10);
        assert_eq!(
            create_entity.occupied_rows.len() + create_entity.occupied_cols.len(),
            20
        );

        let check_v1_entity = Check {
            coords: vec![Coord { col: 0, row: 0 }, Coord { col: 1, row: 1 }],
        };
        let check_v1_str = serde_yaml::to_string(&check_v1_entity).unwrap();
        let check_v1_resp = app
            .post(format!("/game/{}/check", game_setup_id))
            .body_string(check_v1_str)
            .recv_string()
            .await
            .unwrap();
        assert_eq!(check_v1_resp, "Try harder!");

        let check_v1_entity = Check {
            coords: vec![
                Coord { col: 0, row: 1 },
                Coord { col: 0, row: 4 },
                Coord { col: 2, row: 3 },
                Coord { col: 3, row: 6 },
                Coord { col: 3, row: 7 },
                Coord { col: 3, row: 8 },
                Coord { col: 5, row: 0 },
                Coord { col: 5, row: 2 },
                Coord { col: 5, row: 3 },
                Coord { col: 5, row: 4 },
                Coord { col: 5, row: 6 },
                Coord { col: 6, row: 0 },
                Coord { col: 7, row: 0 },
                Coord { col: 7, row: 7 },
                Coord { col: 7, row: 9 },
                Coord { col: 8, row: 0 },
                Coord { col: 8, row: 7 },
                Coord { col: 8, row: 9 },
                Coord { col: 9, row: 4 },
                Coord { col: 9, row: 5 },
            ],
        };
        let check_v1_str = serde_yaml::to_string(&check_v1_entity).unwrap();
        let check_v1_resp = app
            .post(format!("/game/{}/check", game_setup_id))
            .body_string(check_v1_str)
            .recv_string()
            .await
            .unwrap();
        assert_eq!(check_v1_resp, "Well done!");
    }

    #[async_std::test]
    async fn test_hints() {
        let game_setup_id = uuid::uuid!("37073150-f43d-4609-94ec-dcbeffcb472a");
        let app = build_app();
        let hint_resp = app
            .get(format!("/game/{}/hint?limit=3", game_setup_id))
            .recv_string()
            .await
            .unwrap();
        let hint_entity = serde_yaml::from_str::<Hints>(&hint_resp).unwrap();
        assert_eq!(hint_entity.coords.len(), 3);
        assert_eq!(
            hint_entity,
            Hints {
                coords: vec![
                    Coord { row: 6, col: 4 },
                    Coord { row: 6, col: 5 },
                    Coord { row: 6, col: 6 }
                ]
            }
        );
    }
    #[async_std::test]
    async fn test_create_nested() {
        let game_setup_id = uuid::uuid!("37073150-f43d-4609-94ec-dcbeffcb472a");
        let app = build_app();
        let create_resp = app
            .post(format!("/game/{}", game_setup_id))
            .recv_string()
            .await
            .unwrap();
        let create_entity = serde_yaml::from_str::<NestedNewGame>(&create_resp).unwrap();
        assert_eq!(create_entity.number_of_hints, 10);
        assert_eq!(create_entity.occupied_cols.unwrap().head, 0);
        assert_eq!(create_entity.occupied_rows.unwrap().head, 1);
    }

    #[async_std::test]
    async fn test_hints_nested() {
        let game_setup_id = uuid::uuid!("dd8fb490-72c8-485b-aeea-537b9be34e4b");
        let app = build_app();
        let hint_resp = app
            .get(format!("/game/{}/hint?limit=3", game_setup_id))
            .recv_string()
            .await
            .unwrap();
        let hint_entity = serde_yaml::from_str::<NestedHints>(&hint_resp).unwrap();
        assert_eq!(
            hint_entity,
            NestedHints {
                coords: Some(NonEmptyList {
                    head: Coord { row: 0, col: 8 },
                    tail: Some(Box::new(NonEmptyList {
                        head: Coord { row: 0, col: 7 },
                        tail: Some(Box::new(NonEmptyList {
                            head: Coord { row: 0, col: 6 },
                            tail: None
                        }))
                    }))
                })
            }
        );
    }
}
