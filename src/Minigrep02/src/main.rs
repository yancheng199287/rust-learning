use std::env;
use std::process;

use Minigrep02;
use Minigrep02::Config;


/**
  使用迭代器，最终完美版本
**/
fn main() {
    println!("Hello, 本案例演示了如何将代码拆分到库!");


    let config = Config::new(env::args()).unwrap_or_else(|err| {
        //cargo run to poem.txt > output.txt  标准输出 重定向到文件中，不会出现在控制台
        eprintln!("Problem parsing arguments: {}", err);
        // println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = Minigrep02::run(config) {
        eprintln!("Application error: {}", e);
        // println!("Application error: {}", e);
        process::exit(1);
    }
}
