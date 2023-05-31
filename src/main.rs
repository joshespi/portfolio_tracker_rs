#[macro_use]
extern crate rocket;

use rocket::serde::Serialize;
use rocket::serde::json::Json;
use std::fs;
use serde::Deserialize; // Import the Deserialize trait

#[derive(Serialize, Deserialize)] // Implement Deserialize trait for deserialization
struct Asset {
    name: String,
    quantity: f64,
    price: f64,
}

#[derive(Serialize, Deserialize)] // Implement Deserialize trait for deserialization
struct Portfolio {
    name: String,
    assets: Vec<Asset>,
}

impl Portfolio {
    fn new(name: &str) -> Portfolio {
        Portfolio {
            name: name.to_string(),
            assets: Vec::new(),
        }
    }

    fn add_asset(&mut self, name: &str, quantity: f64, price: f64) {
        let asset = Asset {
            name: name.to_string(),
            quantity,
            price,
        };
        self.assets.push(asset);
    }
}

#[get("/")]
fn index() -> &'static str {
    "Welcome to the portfolio tracker!"
}

#[get("/portfolio")]
fn get_portfolio() -> Result<Json<Portfolio>, String> {
    let file_content = fs::read_to_string("portfolio.json")
        .map_err(|e| format!("Failed to read portfolio file: {}", e))?;

    let portfolio: Portfolio = serde_json::from_str(&file_content)
        .map_err(|e| format!("Failed to deserialize portfolio JSON: {}", e))?;

    Ok(Json(portfolio))
}


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, get_portfolio])
}
