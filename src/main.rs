use rocket::http::ContentType;
use rocket::serde::{Deserialize, Serialize};
use rocket::{get, routes};
use tera::Context;

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
    ticker: String,
    holdings: Vec<CryptoHolding>,
}

#[derive(Debug, Deserialize, Serialize)]
struct StockPortfolio {
    ticker: String,
    holdings: Vec<StockHolding>,
}

#[derive(Debug, Deserialize, Serialize)]
struct CryptoHolding {
    exchange: String,
    quantity: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct StockHolding {
    account: String,
    quantity: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct VehicleAsset {
    name: String,
    #[serde(rename = "Current_Value")]
    current_value: f64,
}

#[derive(Debug, Deserialize, Serialize)]
struct RealEstateAsset {
    name: String,
    #[serde(rename = "Current_Value")]
    current_value: f64,
}

#[derive(Debug, Deserialize, Serialize)]
struct Debt {
    name: String,
    amount_owed: String,
}

#[get("/")]
fn get_portfolio() -> (ContentType, String) {
    // Read the portfolio data from a JSON file
    let portfolio: Portfolio = serde_json::from_str(include_str!("portfolio.json"))
        .expect("Failed to read portfolio data");

    let mut context = Context::new();
    context.insert("portfolio", &portfolio);

    // Render the HTML template and pass the portfolio data to it
    let html_content = tera::Tera::new("templates/**/*")
        .expect("Failed to create Tera instance")
        .render("index.html", &context)
        .expect("Failed to render HTML template");

    (ContentType::HTML, html_content)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![get_portfolio])
}
