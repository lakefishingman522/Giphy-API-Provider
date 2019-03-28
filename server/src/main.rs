mod app;
mod config;
mod db;
mod handlers;
mod jwt;
mod models;

use std::sync::Arc;

use actix::prelude::*;
use reqwest::r#async::Client;
use env_logger;
use log::info;
use wither::prelude::*;

use crate::{
    app::new_app,
    db::MongoExecutor,
    models::{SavedGif, User},
};

fn main() {
    let cfg = Arc::new(config::Config::new());
    let _ = env_logger::init();

    // Build HTTP client.
    let client = Client::new();

    // Connect to DB backend & sync models.
    let db = MongoExecutor::new(&*cfg).expect("Unable to connect to database backend.");
    info!("Synchronizing data models.");
    User::sync(db.0.clone()).expect("Faild to sync User model.");
    SavedGif::sync(db.0.clone()).expect("Faild to sync SavedGif model.");

    // Boot the various actors of this system.
    let sys = actix::System::new("api");
    let db_executor = SyncArbiter::start(4, move || db.clone());
    let _server = new_app(db_executor, client, cfg);
    let _ = sys.run();
}
