fn main() {
    println!("Hello, 本案例演示了泛型!");
    compare_num();
}


fn compare_num() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);
    assert_eq!(result, 100);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&number_list);
    println!("The largest number is {}", result);
    assert_eq!(result, 6000);


    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 6.0 };
}


//这个函数有一个参数 list，它代表会传递给函数的任何具体的 i32值的 slice。函数定义中的 list 代表任何 &[i32]
/*找出重复代码。
将重复代码提取到了一个函数中，并在函数签名中指定了代码中的输入和返回值。
将重复代码的两个实例，改为调用函数。
在不同的场景使用不同的方式，我们也可以利用相同的步骤和泛型来减少重复代码。与函数体可以在抽象list而不是特定值上操作的方式相同，泛型允许代码对抽象类型进行操作。
*/
fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

//Point 的定义中只使用了一个泛型类型，这个定义表明结构体 Point 对于一些类型 T 是泛型的，而且字段 x 和 y 都是 相同类型的
struct Point<T> {
    x: T,
    y: T,
}

//注意必须在 impl 后面声明 T，这样就可以在 Point<T> 上实现的方法中使用它了。
// 在 impl 之后声明泛型 T ，这样 Rust 就知道 Point 的尖括号中的类型是泛型而不是具体类型。
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// 也可以定义一个非泛型的方法，指定具体的类型
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

//在结构体中定义多个泛型参数
struct Point1<T, U> {
    x: T,
    y: U,
}

// 在枚举中定义一个泛型参数
enum Option<T> {
    Some(T),
    None,
}