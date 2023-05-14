
use std::env;
use std::process;

use log::{error};

use rlox::Config;


fn main() {
    flexi_logger::Logger::try_with_str("
            info, 
            rlox::lox::parser=debug, 
            rlox::lox::builder=debug,
            rlox::lox::vm=debug
        ")
        .expect("flexi_logger configuration failed")
        .start()
        .expect("flexi_logger start failed");

    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        error!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = rlox::run(config) {
        error!("ERROR: {}", e);

        process::exit(1);
    }
}

