use rocket::{get, routes};
use rocket::serde::{Serialize, Deserialize};
use rocket::serde::json::Json;

#[macro_use] extern crate rocket;


#[derive(Debug, Deserialize, Serialize)]
struct Portfolio {
    name: String,
    holdings: Vec<Holding>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Holding {
    symbol: String,
    quantity: u32,
}

#[get("/")]
fn get_portfolio() -> Json<Portfolio> {
    // Read the portfolio data from a JSON file
    let portfolio: Portfolio = serde_json::from_str(include_str!("portfolio.json"))
        .expect("Failed to read portfolio data");

    Json(portfolio)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![get_portfolio])
}
