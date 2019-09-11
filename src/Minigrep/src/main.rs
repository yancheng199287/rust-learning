/*，当所需函数嵌套了多于一层模块时，通常将父模块引入作用域，而不是其自身
,这比增加了 use std::env::args; 后仅仅使用 args 调用函数要更明确一些，因为 args 容易被错认成一个定义于当前模块的函数。
*/
use std::env;

//文件读取
use std::fs;


use std::process;
use std::error::Error;

fn main() {
    println!("Hello, 本案例演示一个搜索文件文件内容的小工具!");

    let args: Vec<String> = env::args().collect();

    /*  let config = Config::new(&args);
      println!("program name is {:?}", config.program);
      println!("Searching for {}", config.query);
      println!("In file {}", config.filename);
      */

    /* 使用 unwrap_or_else 可以进行一些自定义的非 panic! 的错误处理。当 Result 是 Ok 时，这个方法的行为类似于 unwrap：它返回 Ok 内部封装的值。
     然而，当其值是 Err 时，该方法会调用一个 闭包（closure），也就是一个我们定义的作为参数传递给 unwrap_or_else 的匿名函数。
     */
    let config = Config::new1(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        //process::exit 会立即停止程序并将传递给它的数字作为退出状态码
        process::exit(1);
    });

    //使用 if let 来检查 run 是否返回一个 Err 值 因为 run 在成功时返回 ()，而我们只关心检测错误，所以并不需要 unwrap_or_else 来返回未封装的值，因为它只会是 ()。
    if let Err(e)=run(config)    {
        println!("Application error: {}", e);
        process::exit(1);
    }
}

//Box<dyn Error> 意味着函数会返回实现了 Error trait 的类型，不过无需指定具体将会返回的值的类型。这提供了在不同的错误场景可能有不同类型的错误返回值的灵活性。
// 这也就是 dyn，它是 “动态的”（“dynamic”）的缩写。
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    //不同于遇到错误就 panic!，? 会从函数中返回错误值并让调用者来处理它。
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);

// run 函数签名中声明成功类型返回值是 ()，这意味着需要将 unit 类型值包装进 Ok 值中。
// Ok(()) 一开始看起来有点奇怪，不过这样使用 () 是表明我们调用 run 只是为了它的副作用的惯用方式；它并没有返回什么有意义的值。
    Ok(())
}


struct Config {
    program: String,
    query: String,
    filename: String,
}

impl Config {
    fn new1(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let program = args[0].clone();
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { program, query, filename })
    }


    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("not enough arguments");
        }

        let program = args[0].clone();
        let query = args[1].clone();
        let filename = args[2].clone();
        Config { program, query, filename }
    }
}
