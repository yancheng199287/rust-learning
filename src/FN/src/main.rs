fn main() {
    println!("Hello, 本案例学习 方法语法");

    let rect1 = Rectangle {
        width: 100,
        height: 80,
    };

    let area = rect1.area();
    println!("this rectangle area is {}", area);


    let rect2 = Rectangle {
        width: 50,
        height: 40,
    };

    let can_hold = rect1.can_hold(&rect2);
    print!("rect1 can hold rect2 :{}", can_hold);

    let rect3 = Rectangle::square(30);
    println!("rect3 area is {}", rect3.area());
}


struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {

    // impl 块中定义，以 self 作为参数的函数 叫实例方法，必须声明对象之后调用， 不 以 self 作为参数的函数 叫关联函数，类似java静态函数，可以使用结构体::调用


    //因为该方法位于 impl Rectangle 上下文中所以 Rust 知道 self 的类型是 Rectangle
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // self 后面可以接多个参数
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

