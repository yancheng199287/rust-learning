/*
当创建不可变和可变引用时，我们分别使用 & 和 &mut 语法。对于 RefCell<T> 来说，则是 borrow 和 borrow_mut 方法，这属于 RefCell<T> 安全 API 的一部分。

borrow 方法返回 Ref 类型的智能指针，borrow_mut 方法返回 RefMut 类型的智能指针。这两个类型都实现了 Deref 所以可以当作常规引用对待。

RefCell<T> 记录当前有多少个活动的 Ref<T> 和 RefMut<T> 智能指针。每次调用 borrow，RefCell<T> 将活动的不可变借用计数加一。
当 Ref 值离开作用域时，不可变借用计数减一。

就像编译时借用规则一样，RefCell<T> 在任何时候只允许有多个不可变借用或一个可变借用。

如果我们尝试违反这些规则，相比引用时的编译时错误，RefCell<T> 的实现会在运行时 panic!
*/
fn main() {
    println!("Hello, 本案例演示了 RefCell<T> 和内部可变性模式!");
  //  have_more_mub_data_owner();
    loop_reference();
}


// RefCell<T> 允许在运行时执行可变借用检查，所以我们可以在即便 RefCell<T> 自身是不可变的情况下修改其内部的值
//在不可变值内部改变值就是 内部可变性 模式
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: 'a + Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
    where T: Messenger {
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 0.75 && percentage_of_max < 0.9 {
            self.messenger.send("Warning: You've used up over 75% of your quota!");
        } else if percentage_of_max >= 0.9 && percentage_of_max < 1.0 {
            self.messenger.send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        //现在 sent_messages 字段的类型是 RefCell<Vec<String>> 而不是 Vec<String>
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            // MockMessenger { sent_messages: vec![] }
            MockMessenger { sent_messages: RefCell::new(vec![]) }
        }
    }


    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            //不能修改 MockMessenger 来记录消息，因为 send 方法获取 self 的不可变引用。
            // 我们也不能参考错误文本的建议使用 &mut self 替代，因为这样 send 的签名就不符合 Messenger trait 定义中的签名了

            //对于 send 方法的实现，第一个参数仍为 self 的不可变借用，这是符合方法定义的。
            // 我们调用 self.sent_messages 中 RefCell 的 borrow_mut 方法来获取 RefCell 中值的可变引用，这是一个 vector。
            // 接着可以对 vector 的可变引用调用 push 以便记录测试过程中看到的消息。
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        //为了看到其内部 vector 中有多少个项，需要调用 RefCell 的 borrow 以获取 vector 的不可变引用。
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}



//结合 Rc<T> 和 RefCell<T> 来拥有多个可变数据所有者

fn have_more_mub_data_owner() {

    #[derive(Debug)]
    enum List {
        //RefCell<T> 的一个常见用法是与 Rc<T> 结合。回忆一下 Rc<T> 允许对相同数据有多个所有者，不过只能提供数据的不可变访问。
        //如果有一个储存了 RefCell<T> 的 Rc<T> 的话，就可以得到有多个所有者 并且 可以修改的值了
        Cons(Rc<RefCell<i32>>, Rc<List>),
        Nil,
    }

    use List::{Cons, Nil};
    use std::rc::Rc;
    use std::cell::RefCell;

    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}

//循环引用
fn loop_reference(){
    use std::rc::Rc;
    use std::cell::RefCell;
    use List::{Cons, Nil};

    #[derive(Debug)]
    enum List {
        Cons(i32, RefCell<Rc<List>>),
        Nil,
    }

    impl List {
        fn tail(&self) -> Option<&RefCell<Rc<List>>> {
            match self {
                Cons(_, item) => Some(item),
                Nil => None,
            }
        }
    }

    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // 取消如下行的注释来观察引用循环；
    // 这会导致栈溢出
   //  println!("a next item = {:?}", a.tail());
}