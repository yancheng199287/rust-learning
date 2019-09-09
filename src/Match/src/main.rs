fn main() {
    println!("Hello, 本案例演示了 match 控制流运算符!");

    let coin = Coin::Penny;
    let money = value_in_cents(coin);
    println!("获取的Penny的面值是：{}", money);


    let coin1 = Coin::Quarter(UsState::Alabama);
    let money = value_in_cents(coin1);
    println!("获取的Penny的面值是：{}", money);


    let count = plus_one(Some(10));
    println!("匹配Option，加+1后的值是：{}", count.unwrap());


    let some_u8_value = Some(0u8);
    // match的语法糖 更加简洁
    if let Some(3) = some_u8_value {
        println!("yes");
    } else {
        println!("no");
    }
}

enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            // 匹配枚举内部的枚举值
            match state {
                UsState::Alabama => println!("1 State quarter from {:?}!", state),
                UsState::Alaska => println!("1 State quarter from {:?}!", state)
            }
            25
        }
    }
}


fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}


impl std::fmt::Debug for UsState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            UsState::Alabama => f.pad("Alabama"),
            UsState::Alaska => f.pad("Alaska"),
        }
    }
}