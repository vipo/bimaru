

use serde::Serialize;
use tide::Request;
use tide::Response;
use uuid::Uuid;

#[derive(Debug, Serialize)]
#[warn(dead_code)]
struct NewGame {
    id: String,
    game_number: u32,
    number_of_hints: u8,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();
    app.at("/game/:num").post(new_game);
    app.listen("0.0.0.0:8080").await?;
    Ok(())
}

async fn new_game(req: Request<()>) -> tide::Result {
    let game_num: u32 = str::parse::<u32>(req.param("num")?)?;
    let response_entity: NewGame = NewGame{
        id: Uuid::new_v4().to_string(),
        game_number: game_num,
        number_of_hints: 20
    };
    let response: Response = yaml_response(201, &response_entity)?;
    Ok(response)
}

fn yaml_response<T>(status: u16, value: &T) -> tide::Result
where
    T: Serialize,
{
    let yaml: String = serde_yaml::to_string(&value)?;
    Ok(Response::builder(status)
        .body(yaml)
        .content_type("text/x-yaml")
        .build()
    )  
}