extern crate rust_minigrep;

use std::env;
use std::process;

use rust_minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect(); // コマンドライン引数をベクタに格納

    let config = Config::new(&args).unwrap_or_else(|err| {
        // 引数解析時に問題
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = rust_minigrep::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
