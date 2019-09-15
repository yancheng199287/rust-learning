use crate::List::{Cons, Nil};
use std::ops::Deref;

fn main() {
    println!("Hello, 本案例我们将演示智能指针!");
    // save_data_in_heap();
    // cons_list_in_heap();
    // dereference_operator();
    // diy_pointer();
    // drop_resource();
   // reference_counting_pointer();
}



//Rc<T> 引用计数智能指针
//使用 Rc 允许一个值有多个所有者，引用计数则确保只要任何所有者依然存在，其值也保持有效。
fn reference_counting_pointer() {
    /*  enum List1 {
          Cons(i32, Box<List1>),
          Nil,
      }

      use List1::{Cons, Nil};

      let a = Cons(5,
                   Box::new(Cons(10,
                                 Box::new(Nil))));
      //Cons 成员拥有其储存的数据，所以当创建 b 列表时，a 被移动进了 b 这样 b 就拥有了 a
      let b = Cons(3, Box::new(a));

      //当再次尝使用 a 创建 c 时，这不被允许因为 a 的所有权已经被移动
      let c = Cons(4, Box::new(a));*/


    enum List2 {
        Cons(i32, Rc<List2>),
        Nil,
    }

    use List2::{Cons, Nil};

    //需要使用 use 语句将 Rc<T> 引入作用域因为它不在 prelude 中
    use std::rc::Rc;

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));

    //Rc::clone 的实现并不像大部分类型的 clone 实现那样对所有数据进行深拷贝。Rc::clone 只会增加引用计数，这并不会花费多少时间。
    // 深拷贝可能会花费很长时间。通过使用 Rc::clone 进行引用计数，可以明显的区别深拷贝类的克隆和增加引用计数类的克隆。

    //当创建 b 时，不同于获取 a 的所有权，这里会克隆 a 所包含的 Rc，这会将引用计数从 1 增加到 2 并允许 a 和 b 共享 Rc 中数据的所有权。
    // 创建 c 时也会克隆 a，这会将引用计数从 2 增加为 3。每次调用 Rc::clone，Rc 中数据的引用计数都会增加，直到有零个以上引用之前其数据都不会被清理。
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));




    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }

    //我们能够看到 a 中 Rc<List> 的初始引用计数为一，接着每次调用 clone，计数会增加一。
    // 当 c 离开作用域时，计数减一。
    // 不必像调用 Rc::clone 增加引用计数那样调用一个函数来减少计数；Drop trait 的实现当 Rc<T> 值离开作用域时自动减少引用计数。
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}//在 main 的结尾当 b 然后是 a 离开作用域时，此处计数会是 0，同时 Rc 被完全清理。


//使用 Drop Trait 运行清理代码
fn drop_resource() {
    struct CustomSmartPointer {
        data: String,
    }

    /*
    Drop trait 包含在 prelude 中，所以无需导入它。我们在 CustomSmartPointer 上实现了 Drop trait，并提供了一个调用 println! 的 drop 方法实现。
    drop 函数体是放置任何当类型实例离开作用域时期望运行的逻辑的地方
    */
    impl Drop for CustomSmartPointer {
        fn drop(&mut self) {
            println!("Dropping CustomSmartPointer with data `{}`!", self.data);
        }
    }

    let c = CustomSmartPointer { data: String::from("my stuff") };
    let d = CustomSmartPointer { data: String::from("other stuff") };
    println!("CustomSmartPointers created.");
}//当其实例离开作用域时会执行 drop方法  一般用来释放打开链接的资源，不允许我们显式调用 drop
// 如果想要手动调用，使用 std::mem::drop ，这个函数不同于 Drop trait 中的 drop 方法。可以通过传递希望提早强制丢弃的值作为参数。


//自定义智能指针
fn diy_pointer() {
    //定义了一个结构体 MyBox 并声明了一个泛型参数 T，因为我们希望其可以存放任何类型的值。
    struct MyBox<T>(T);

    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }

    //  实现 Deref 用于自定义解引用
    impl<T> Deref for MyBox<T> {
        //定义了用于此 trait 的关联类型
        type Target = T;

        /*
        没有 Deref trait 的话，编译器只会解引用 & 引用类型。
        deref 方法向编译器提供了获取任何实现了 Deref trait 的类型的值并调用这个类型的 deref 方法来获取一个它知道如何解引用的 & 引用的能力。
        */
        fn deref(&self) -> &T {
            &self.0
        }
    }

    fn hello(name: &str) {
        println!("Hello, {}!", name);
    }

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);

    //MyBox<T> 类型不能解引用,因为我们并没有为其实现这个功能 这里编译会报错
    //为了启用 * 运算符的解引用功能，需要实现 Deref trait
    assert_eq!(5, *y);


    let m = MyBox::new(String::from("Rust"));
    //Rust 可以通过 deref 调用将 &MyBox<String> 变为 &String。标准库中提供了 String 上的 Deref 实现，其会返回字符串 slice
    //Rust 再次调用 deref 将 &String 变为 &str，这就符合 hello 函数的参数定义
    hello(&m);

    /*
    (*m) 将 MyBox<String> 解引用为 String。接着 & 和 [..] 获取了整个 String 的字符串 slice 来匹配 hello 的签名。
    没有解引用强制多态所有这些符号混在一起将更难以读写和理解。解引用强制多态使得 Rust 自动的帮我们处理这些转换。

    当所涉及到的类型定义了 Deref trait，Rust 会分析这些类型并使用任意多次 Deref::deref 调用以获得匹配参数的类型。
    这些解析都发生在编译时，所以利用解引用强制多态并没有运行时惩罚！
    */
    hello(&(*m)[..]);

    hello(String::from("还有一种写法").as_ref());
    hello(String::from("还有一种写法")[..].as_ref());
    hello(&String::from("还有一种写法"));
}


fn dereference_operator() {
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    // *是解引用符号，一旦解引用了 y，就可以访问 y 所指向的整型值并可以与 5 做比较。
    assert_eq!(5, *y);

    let x = 5;
    //使用解引用运算符以 y 为引用时相同的方式追踪 box 的指针
    let y = Box::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);
}


/*
最简单直接的智能指针是 box，其类型是 Box<T>。 box 允许你将一个值放在堆上而不是栈上。留在栈上的则是指向堆数据的指针。
除了数据被储存在堆上而不是栈上之外，box 没有性能损失。不过也没有很多额外的功能。它们多用于如下场景：

1. 当有一个在编译时未知大小的类型，而又想要在需要确切大小的上下文中使用这个类型值的时候
2. 当有大量数据并希望在确保数据不被拷贝的情况下转移所有权的时候
3. 当希望拥有一个值并只关心它的类型是否实现了特定 trait 而不是其具体类型的时候
*/
fn save_data_in_heap() {
    //这里定义了变量 b，其值是一个指向被分配在堆上的值 5 的 Box
    //将一个单独的值存放在堆上并不是很有意义 这样单独使用 box 并不常见。将像单个 i32 这样的值储存在栈上，
    // 也就是其默认存放的地方在大部分使用场景中更为合适
    // 这里仅仅作为一种演示
    let b = Box::new(5);
    println!("b = {}", b);
}


/*
cons list 的每一项都包含两个元素：当前项的值和下一项。其最后一项值包含一个叫做 Nil 的值并没有下一项。
cons list 通过递归调用 cons 函数产生。代表递归的终止条件（base case）的规范名称是 Nil，它宣布列表的终止。
注意这不同于第六章中的 “null” 或 “nil” 的概念，他们代表无效或缺失的值。
*/
pub enum List {
    //因为 Box<T> 是一个指针，我们总是知道它需要多少空间：指针的大小并不会根据其指向的数据量而改变。
    // 这意味着可以将 Box 放入 Cons 成员中而不是直接存放另一个 List 值。Box 会指向另一个位于堆上的 List 值，而不是存放在 Cons 成员中
    // 这样做，为了避免编译报错，因为如果不用Box 编译器无法分配未知的空间，这是一个循环递归的数据结构，不知何时结束
    Cons(i32, Box<List>),
    Nil,
}

fn cons_list_in_heap() {
    //  let list = Cons(1, Cons(2, Cons(3, Nil)));

    let list = Cons(1,
                    Box::new(Cons(2,
                                  Box::new(Cons(3,
                                                Box::new(Nil))))));
}