mod user;
//告诉 Rust 在另一个与模块同名的文件中加载模块的内容
mod student;
mod law;//告诉 Rust 在另一个与模块同名的文件中加载模块的内容,这里可以单独建一个文件夹


fn main() {
    println!("Hello, 本案例演示，多模块如何分散在不同文件里，并为之main中调用!");
    user::person::walk();
    student::read();
    law::say();
}
