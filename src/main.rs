#[macro_use]
extern crate rocket;

use rocket::figment::Figment;
use rocket::tokio::fs;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use tokio::fs::File;

#[derive(Serialize, Deserialize)]
struct Asset {
    symbol: String,
    name: String,
    quantity: f64,
    price: f64,
}

#[derive(Serialize, Deserialize)]
struct Portfolio {
    assets: Vec<Asset>,
}

impl Portfolio {
    fn new() -> Portfolio {
        Portfolio {
            assets: Vec::new(),
        }
    }
}

#[get("/")]
fn index() -> &'static str {
    "Welcome to the portfolio tracker!"
}

#[get("/portfolio")]
async fn get_portfolio() -> Json<Portfolio> {
    println!("Reading portfolio file...");
    let file_content = match fs::read_to_string("portfolio.json").await {
        Ok(content) => content,
        Err(_) => {
            eprintln!("Error reading file");
            return Json(Portfolio::new());
        }
    };

    let portfolio: Portfolio = match serde_json::from_str(&file_content) {
        Ok(portfolio) => portfolio,
        Err(_) => {
            eprintln!("Error deserializing JSON");
            return Json(Portfolio::new());
        }
    };

    Json(portfolio)
}


#[rocket::main]
async fn main() {
    let figment = Figment::from(rocket::Config::default());
    let rocket = rocket::custom(figment)
        .mount("/", routes![index, get_portfolio]);

    rocket.launch().await.unwrap();
}
