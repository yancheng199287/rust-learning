use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    //println!("With text:\n{}", contents);

    for line in search(&config.query, &contents) {
        println!("{}", line);
    }
    Ok(())
}

pub struct Config {
    program: String,
    query: String,
    filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let program = args[0].clone();
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { program, query, filename })
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe,fast,productive.
Pick three.";

        assert_eq!(
            vec!["safe,fast,productive."],
            search(query, contents)
        );
    }
}


fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    //vec![]
    // 创建一个vector 用来保存搜索的行内容
    let mut results = Vec::new();

    //contents.lines() 获取迭代器 进行循环每行的内容
    for line in contents.lines() {
        // 如果当前行包含指定查询的字符串
        if line.contains(query) {
            //将当前行的内容添加到Result中
            results.push(line);
        }
    }
    results
}
