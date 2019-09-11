/*
迭代器模式允许你对一个项的序列进行某些处理。迭代器（iterator）负责遍历序列中的每一项和决定序列何时结束的逻辑。
当使用迭代器时，我们无需重新实现这些逻辑。
*/


fn main() {
    println!("Hello, world!");
}

#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    //当其遍历每一个项时，它将每一个项加总到一个总和并在迭代完成时返回总和
    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
}


#[test]
fn iterator_demonstration() {
    let mut v1 = vec![1, 2, 3];

    //v1.iter() next 调用中得到的值是 vector 的不可变引用
    // v1.iter_mut(); 迭代可变引用
    //v1.into_iter()所有权并返回拥有所有权的迭代器
    let mut v1_iter = v1.iter();

    //从 next 调用中得到的值是 vector 的不可变引用
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}


/*
一旦 for 循环开始使用 v1_iter，接着迭代器中的每一个元素被用于循环的一次迭代，这会打印出其每一个值：
*/
fn iterator1() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    for val in v1_iter {
        println!("Got: {}", val);
    }
}