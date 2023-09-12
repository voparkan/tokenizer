extern crate exitcode;

extern crate lazy_static;

mod controllers;
mod repl;
mod utils;


use controllers::btleplug;
use repl::Repl;
use std::error::Error;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let mut bt = btleplug::BtleplugController::new().await;
    let mut repl = Repl::new(&mut bt).await;

    repl.start().await
}
