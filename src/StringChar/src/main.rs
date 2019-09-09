/*String 的类型是由标准库提供的，而没有写进核心语言部分，它是可增长的、可变的、有所有权的、UTF-8 编码的字符串类型。
当 Rustacean 们谈到 Rust 的 “字符串”时，它们通常指的是 String 和字符串 slice &str 类型，而不仅仅是其中之一。
虽然本部分内容大多是关于 String 的，不过这两个类型在 Rust 标准库中都被广泛使用，String 和字符串 slice 都是 UTF-8 编码的。
*/

fn main() {
    println!("Hello, world!");

    //新建了一个叫做 s 的空的字符串，接着我们可以向其中装载数据
    let mut s = String::new();


    //String::from 和 .to_string 最终做了完全相同的工作

    let data = "initial contents";

    //使用 to_string 方法从字符串字面值创建 String类型
    let s = data.to_string();
// 该方法也可直接用于字符串字面值：
    let s = "initial contents".to_string();
}

