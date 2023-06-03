use rocket::{get, routes};
use rocket::serde::{Serialize, Deserialize};
use rocket::serde::json::Json;

#[macro_use]
extern crate rocket;

#[derive(Debug, Deserialize, Serialize)]
struct Portfolio {
    crypt_portfolio: Vec<CryptoPortfolio>,
    stock_portfolio: Vec<StockPortfolio>,
    vehicle_assets: Vec<VehicleAsset>,
    realestate_assets: Vec<RealEstateAsset>,
    debts: Vec<Debt>,
}

#[derive(Debug, Deserialize, Serialize)]
struct CryptoPortfolio {
    Ticker: String,
    Holdings: Vec<CryptoHolding>,
}

#[derive(Debug, Deserialize, Serialize)]
struct StockPortfolio {
    Ticker: String,
    Holdings: Vec<StockHolding>,
}

#[derive(Debug, Deserialize, Serialize)]
struct CryptoHolding {
    Exchange: String,
    Quantity: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct StockHolding {
    Account: String,
    Quantity: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct VehicleAsset {
    Name: String,
    #[serde(rename = "Current Value")]
    CurrentValue: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct RealEstateAsset {
    Name: String,
    #[serde(rename = "Current Value")]
    CurrentValue: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Debt {
    name: String,
    amount_owed: String,
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
