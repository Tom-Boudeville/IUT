use structopt::StructOpt;
use anyhow::Result;

mod configuration;
mod app_builder;

use configuration::Configuration;
use app_builder::run_app;
fn main() {
    let config = Configuration::from_args(); // Utiliser from_args() pour créer une instance de Configuration
    run_app(config);
}
