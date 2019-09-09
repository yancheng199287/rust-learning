use crate::sound::instrument::woodwind;//绝对路径

use self::sound::instrument::woodwind as wd;//相对路径 self表示当前路径 as 可以指定别名

use crate::sound::instrument::woodwind::clarinet;//这种是不推荐的，隐藏了模块，容易造成误会是本模块定义的函数

use std::collections::HashMap;// 一些标准库常用的数据类型 却是被推荐直接引入的，这是约定俗成的习惯

use std::{cmp::Ordering, io};


fn main() {
    println!("Hello, 本案例学习如何包 模块 路径 作用域!");

    //如果想要调用函数，需要知道其 路径
    // 路径 可以有两种形式：
    //  绝对路径（absolute path）从 crate 根开始，以 crate 名或者字面值 crate 开头。
    crate::sound::instrument::woodwind::clarinet();

    // 相对路径（relative path）从当前模块开始，以 self、super 或当前模块的标识符开头。
    sound::instrument::woodwind::clarinet();

    let mut v = plant::Vegetable::new("squash");
    v.name = String::from("butternut squash");
    print!("{} are delicious", v.name);


    let order1 = menu::Appetizer::Soup;
    let order2 = menu::Appetizer::Salad;


    //使用 use引入模块，避免写长的路径
    woodwind::clarinet();
    wd::clarinet();

    let mut map = HashMap::new();
    map.insert(1, 2);

    performance_group::instrument::woodwind::clarinet();


}

/*所有项（函数、方法、结构体、枚举、模块和常量）默认是私有的。
可以使用 pub 关键字使项变为公有。
不允许使用定义于当前模块的子模块中的私有代码。
允许使用任何定义于父模块或当前模块中的代码。
换句话说，对于没有 pub 关键字的项，当你从当前模块向 “下” 看时是私有的，不过当你向 “上” 看时是公有的

私有性规则适用于结构体、枚举、函数和方法以及模块
*/
mod sound {
    fn guitar() {}

    pub mod instrument {
        pub mod woodwind {
            pub fn clarinet() {
                //super关键字可以访问本模块上一层模块的所有方法，而无需按照模块指定
                super::super::guitar();
                super::super::test();
                //相对路径，从根路径访问
                crate::breathe_in();
            }
        }
    }

    fn test() {}

    mod voice {}
}

fn breathe_in() {
    // 函数体
}


mod plant {
    pub struct Vegetable {
        // 结构体带有公有和私有的字段
        pub name: String,
        id: i32,
    }

    impl Vegetable {
        pub fn new(name: &str) -> Vegetable {
            Vegetable {
                name: String::from(name),
                id: 1,
            }
        }
    }
}


mod menu {
    //如果有一个公有枚举，其所有成员都是公有。只需在 enum 关键词前加上 pub
    pub enum Appetizer {
        Soup,
        Salad,
    }
}


mod performance_group {
    //在模块中使用其他模块，这样模块可以自由组合嵌套调用了
    //注意这里使用pub 可以包种外部直接通过performance_group调用instrument模块
    pub use crate::sound::instrument;

    pub fn clarinet_trio() {
        instrument::woodwind::clarinet();
        instrument::woodwind::clarinet();
        instrument::woodwind::clarinet();
    }
}