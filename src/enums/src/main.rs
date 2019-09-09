fn main() {
    println!("Hello, 本案例演示了枚举的使用!");

    let v4 = IpAddrKind::V4;
    route(v4);

    let home = IpAddress::V4(String::from("127.0.0.1"));
    route1(home);

    let company = IpAddress::V6(192, 168, 1, 2);
    route1(company);


    let msg = Message::Write(String::from("I am message"));
    msg.call();


    let value = Option::Some(String::from("I am a value"));
    let value1 = Some(String::from("I am a other value"));//上面的缩写，主要是太常用了，内置了
    let absent_number: Option<i32> = None;//当有个 None 值时，在某种意义上，它跟空值具有相同的意义：并没有一个有效的值

    let is_none = absent_number.is_some();
    println!("Hello, is_none:{}", is_none);
    println!("Hello, is_none:{}", value1.unwrap());//一般来说，因为这个功能可能会引起恐慌，所以不鼓励使用它。 相反，更喜欢使用模式匹配并明确处理None情况。
}

enum IpAddress {
    V4(String),
    //支持绑定任意类型多个数据，而且每个枚举不必相同
    V6(u8, u8, u8, u8),
}

enum IpAddrKind {
    V4,
    V6,
}

fn route(ip_type: IpAddrKind) {
    match ip_type {
        IpAddrKind::V4 => println!("IP V4 类型地址"),
        IpAddrKind::V6 => println!("IP V6 类型地址")
    }
}

fn route1(ip_address: IpAddress) {
    match ip_address {
        IpAddress::V4(address) => println!("IP V4 类型地址,address:{}", address),
        IpAddress::V6(first, second, third, fourth) => println!("IP V6 类型地址,address:{}.{}.{}.{}", first, second, third, fourth)
    }
}

enum Message {
    //没有关联任何数据
    Quit,
    //包含一个匿名结构体
    Move { x: i32, y: i32 },
    // 包含单独一个 String
    Write(String),
    //包含三个 i32
    ChangeColor(i32, i32, i32),
}

// 枚举也可以定义属于自己的实例方法
impl Message {
    fn call(&self) {
        match self {
            Message::Write(msg) => println!("you have call me，msg：{}", msg),

            _ => { println!("通配符，标识什么都没匹配到") }
        }
    }
}