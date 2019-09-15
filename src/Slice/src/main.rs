fn main() {
    /*    let mut s = String::from("hello world");
        let word = first_word(&s); // word 的值为 5
        s.clear(); // 这清空了字符串，使其等于 ""
        // word 在此处的值仍然是 5，
        // 但是没有更多的字符串让我们可以有效地应用数值 5。word 的值现在完全无效！
        println!("s:{},word:{}", s, word);*/


    let mut s = String::from("hello world");

    /*     这类似于引用整个 String 不过带有额外的 [0..5] 部分。
       它不是对整个 String 的引用，而是对部分 String 的引用。
       start..end 语法代表一个以 start 开头并一直持续到但不包含 end 的 range。
       如果需要包含 end，可以使用 ..= 而不是 ..
       */

    let hello = &s[0..5];
    let world = &s[6..=10];

    first_word(&s);
    println!("s:{},hello:{}，world：{}", s, hello, world);


    // s是一个字面值
    let mut s = String::from("hello world");
    //&s是获取字面值的不可变引用，如果想获取可变引用  & mut s
    let w = first_word_1(&s);

    // 因为要修改字符串，要获取一个可变引用， 在同一个作用域中不能同时获取获取不可变和可变，也不能多次获取可变引用
    s.clear();//会报错， 使用 slice 保证 字符串完整性
    println!("the first word is: {}", w);
}

fn first_word(s: &String) -> usize {
    //as_bytes 方法将 String 转化为字节数组：
    let bytes = s.as_bytes();

    //使用 iter 方法在字节数组上创建一个迭代器：
    //enumerate 包装 iter 的结果并返回一个元组
    //回元组的第一个元素是索引，第二个元素是集合中元素的引用
    for (i, &item) in bytes.iter().enumerate() {
        //b' ' s 是空格
        if item == b' ' {
            return i;
        }
    }
    return s.len();
}


fn first_word_1(s: &str) -> &str {
    &s[0..]
}
