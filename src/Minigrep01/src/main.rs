
use std::env;
use std::process;

use Minigrep01;
use Minigrep01::Config;

fn main() {
    println!("Hello, 本案例演示了如何将代码拆分到库!");

    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        //cargo run to poem.txt > output.txt  标准输出 重定向到文件中，不会出现在控制台
        eprintln!("Problem parsing arguments: {}", err);
       // println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e)=Minigrep01::run(config)    {
        eprintln!("Application error: {}", e);
       // println!("Application error: {}", e);
        process::exit(1);
    }

}
