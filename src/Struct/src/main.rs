fn main() {
    println!("Hello, world!");
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    //注意整个实例必须是可变的,才能改变其值
    user1.email = "648830605@qq.com".parse().unwrap();
    user1.email = String::from("123456@sina.com");
    print!("user1: username:{}", user1.email);

    let user2 = build_user("4666@gmail.com".parse().unwrap(), "Jason".parse().unwrap());
    print!("user2: username:{}", user2.email);


    // 注意 black 和 origin 值的类型不同，因为它们是不同的元组结构体的实例。你定义的每一个结构体有其自己的类型，即使结构体中的字段有着相同的类型
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

 struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}


impl User{

    fn introduce(self){
        println!("{} {}",self.username,self.email)
    }
}