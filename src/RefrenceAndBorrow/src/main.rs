fn main() {
    let s1 = String::from("hello");

    //我们将获取引用作为函数参数称为 借用（borrowing）

    //这些 & 符号就是 引用，它们允许你使用值但不获取其所有权
    let len = calculate_length(&s1);
    println!("The s1 length of '{}' is {}.", s1, len);


    let mut s2 = String::from("hello");
    change(&mut s2);
    println!("The s2 length of '{}'", s2);


    //不过可变引用有一个很大的限制：在特定作用域中的特定数据有且只有一个可变引用。这些代码会失败：
    /*  let mut s3 = String::from("hello");
      let r1 = &mut s3;
      let r2 = &mut s3;
      println!("{}, {}", r1, r2);*/


    //可以使用大括号来创建一个新的作用域，以允许拥有多个可变引用，只是不能 同时 拥有：
    /* let mut s4 = String::from("hello");
     {
         let r1 = &mut s4;
     } // r1 在这里离开了作用域，所以我们完全可以创建一个新的引用

     let r2 = &mut s4;
     println!("r1 在上面的作用域已经消失了，不能在这里打印了，我们可以使用 r2：{}", r2);*/


    /*  //不能在拥有不可变引用的同时拥有可变引用
      let mut s5 = String::from("hello");
      let r1 = &s5; // no problem  不可变引用 可以有多个
      let r2 = &s5; // no problem
      let r3 = &mut s5; // BIG PROBLEM 有问题，上面已经存在不可变引用
      println!("{}, {}, and {}", r1, r2, r3);*/

    let reference_to_nothing = dangle();
}

//// s 是对 String 的引用
fn calculate_length(s: &String) -> usize {
    s.len()
}// 这里，s 离开了作用域。但因为它并不拥有引用值的所有权，所以什么也不会发生


// 想要改变引用的值，那么必须定义可变引用，且引用的变量实参也必须是可变的
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
/*因为 s 是在 dangle 函数内创建的，当 dangle 的代码执行完毕后，s 将被释放。
不过我们尝试返回它的引用。这意味着这个引用会指向一个无效的 String，这可不对！Rust 不会允许我们这么做。
这里的解决方法是直接返回 s  字符串值 ，所有权被移动出去，所以没有值被释放。
*/
fn dangle() -> &String {  // dangle 返回一个字符串的引用
    let s = String::from("hello");// s 是一个新字符串
    &s // 返回字符串 s 的引用
} // 这里 s 离开作用域并被丢弃。其内存被释放。
// 危险！
