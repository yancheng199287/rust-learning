use std::fmt::{Display, Debug};

fn main() {
    println!("Hello, world!");

    let mut article = NewArticle {
        headline: String::from("Rust"),
        location: String::from("american"),
        author: String::from("John "),
        content: String::from(" some description "),
    };

    let mut article1 = NewArticle {
        headline: String::from("Rust1"),
        location: String::from("american"),
        author: String::from("John "),
        content: String::from(" some description "),
    };

    let mut tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());

    //notify(tweet);

    // notify1::<Tweet>(tweet);
    //notify2(article, tweet);
    notify3::<NewArticle>(article, article1);
}

//trait 中文 特征，类似于 Java中的接口，是一组描述多个行为的抽象的类型
pub trait Summary {
    fn summarize(&self) -> String;

    fn default_method(&self) {
        println!("默认实现方法，不需要实现")
    }
}

pub struct NewArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// 为Tweet结构体实现Summary接口，定义关于这个结构体的相关方法的类型
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

//使用 trait 来接受多种不同类型的实例参数
pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

//notify 跟notify1是一样的，是一种语法糖
pub fn notify1<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}

//trait bound 适合 获取多个实现了 Summary 的类型： 适合不同trait的多个实例类型
pub fn notify2(item1: impl Summary, item2: impl Summary) {
    println!("notify2 item1:{},item2:{}", item1.summarize(), item2.summarize());
}

//强制是相同的类型
pub fn notify3<T: Summary>(item1: T, item2: T) {
    println!("notify3 item1:{},item2:{}", item1.summarize(), item2.summarize());
}


//item 就需要同时实现两个不同的 trait：Display 和 Summary。这可以通过 + 语法实现
pub fn notify4(item: impl Summary + Display) {}

//这个语法也适用于泛型的 trait bound：
pub fn notify5<T: Summary + Display>(item: T) {}


fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {}

//使用where 精简写法
fn some_function1<T, U>(t: T, u: U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{}

//返回值中使用 impl Trait 语法，来返回实现了某个 trait 的类型,只适用于返回单一类型,不支持多种实例类型
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

// 这种方式是错误的，无法编译，不支持多种实例类型
/*
fn returns_summarizable1(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from("The Pittsburgh Penguins once again are the best
            hockey team in the NHL."),
        }
    } else {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        }
    }
}*/
