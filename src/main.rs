#[macro_use]
extern crate rocket;
extern crate diesel;
extern crate dotenv;

use dotenv::dotenv;
use rocket::config::Config;
use rocket::{Build, Rocket};

#[launch]
fn rocket() -> Rocket<Build> {
    dotenv().ok(); // Load environment variables from the .env file

    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8000".to_string()) // Default to 8000 if PORT is not set
        .parse::<u16>()
        .unwrap_or(8000); // Fallback to 8000 if parsing fails

    let config = Config {
        port,
        ..Config::default() // Use default values for other settings
    };

    rocket::custom(config) // Return the Rocket instance without `.launch()`
}
