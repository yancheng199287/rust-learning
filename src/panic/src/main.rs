use std::fs::File;
use std::io;
use std::io::{ErrorKind, Read};
/*panic! 宏代表一个程序无法处理的状态，并停止执行而不是使用无效或不正确的值继续处理。Rust 类型系统的 Result 枚举代表操作可能会在一种可以恢复的情况下失败。
可以使用 Result 来告诉代码调用者他需要处理潜在的成功或失败。
在适当的场景使用 panic! 和 Result 将会使你的代码在面对不可避免的错误时显得更加可靠。
*/
fn main() {
    println!("Hello, world!");
    //抛出一个错误 panic!("crash and burn");
    // let v = vec![1, 2, 3];
    // 数组越界错误 v[99];

    let f = File::open("temp01.txt");

    let f = match f {
        Ok(file) => file,
        //匹配到打开文件错误，我们可以根据错误类型再进行匹配
        Err(error) => match error.kind() {
            // 如果文件不存在导致打开错误，那么我们就创建文件
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                // 如果创建失败 就抛出异常
                Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
            },
            other_error => panic!("There was a problem opening the file: {:?}", other_error),
        },
    };

    // unwrap 如果返回值存在那么返回值，如果不存在，调用 panic！宏 打印错误信息
    //let f1=File::open("sayHello.txt").unwrap();

    // expect 自定义错误信息
    //  let f1 = File::open("sayHello.txt").expect("can not open this file!");

    // let result = read_username_from_file();
    // println!("result:{}", result.unwrap());

    let result = read_username_from_file_1();
    println!("result:{}", result.unwrap());
}


fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}


fn read_username_from_file_1() -> Result<String, io::Error> {
    /*

    问号的作用类似这样的作用，只不过是更简单，避免很多样板代码
    File::open 调用结尾的 ? 将会把 Ok 中的值返回给变量 f。如果出现了错误，? 会提早返回整个函数并将一些 Err 值传播给调用者。
    let mut f = match f {
         Ok(file) => file,
         Err(e) => return Err(e),
     };
     */
    let mut f = File::open("hello333.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_2() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}