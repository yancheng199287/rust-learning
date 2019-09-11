/*
迭代器模式允许你对一个项的序列进行某些处理。迭代器（iterator）负责遍历序列中的每一项和决定序列何时结束的逻辑。
当使用迭代器时，我们无需重新实现这些逻辑。
*/


fn main() {
    println!("Hello, world!");
    using_other_iterator_trait_methods();
}

//#[test]
fn using_other_iterator_trait_methods() {
    //注意 zip 只产生四对值；理论上第五对值 (5, None) 从未被产生，因为 zip 在任一输入迭代器返回 None 时也返回 None。
    let sum: u32 = Counter::new().zip(Counter::new().skip(1))
        .map(|(a, b)| {
            println!("a:{} n:{}", a, b);
            a * b
        })
        .filter(|x| x % 3 == 0)
        .sum();
    assert_eq!(18, sum);
}


#[test]
fn calling_next_directly() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}

struct Counter {
    //Counter 结构体有一个字段 count。这个字段存放一个 u32 值，它会记录处理 1 到 5 的迭代过程中的位置
    //count 是私有的因为我们希望 Counter 的实现来管理这个值。
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}


// 自定义迭代器
impl Iterator for Counter {
    //这里将迭代器的关联类型 Item 设置为 u32，意味着迭代器会返回 u32 值集合
    type Item = u32;

    /*这也就是为何将 count 初始化为 0：我们希望迭代器首先返回 1。如果 count 值小于 6，next 会返回封装在 Some 中的当前值，
    不过如果 count 大于或等于 6，迭代器会返回 None。
    */
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}


#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    //shoes_in_my_size 函数调用了 into_iter 来创建一个获取 vector 所有权的迭代器。
    // 接着调用 filter 将这个迭代器适配成一个只含有那些闭包返回 true 的元素的新迭代器。
    shoes.into_iter()
        //闭包从环境中捕获了 shoe_size 变量并使用其值与每一只鞋的大小作比较，只保留指定大小的鞋子。最终，调用 collect 将迭代器适配器返回的值收集进一个 vector 并返回。
        .filter(|s| s.size == shoe_size)
        .collect()
}

#[test]
fn filters_by_size() {
    let shoes = vec![
        Shoe { size: 10, style: String::from("sneaker") },
        Shoe { size: 13, style: String::from("sandal") },
        Shoe { size: 10, style: String::from("boot") },
    ];

    let in_my_size = shoes_in_my_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe { size: 10, style: String::from("sneaker") },
            Shoe { size: 10, style: String::from("boot") },
        ]
    );
}


/*
调用 map 方法创建一个新迭代器，接着调用 collect 方法消费新迭代器并创建一个 vector
因为 map 获取一个闭包，可以指定任何希望在遍历的每个元素上执行的操作。
这是一个展示如何使用闭包来自定义行为同时又复用 Iterator trait 提供的迭代行为的绝佳例子
*/

#[test]
fn iterator11() {
    let v1: Vec<i32> = vec![1, 2, 3];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
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