use std::cell::Cell;
use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    println!("Hello, world!");
}


/**
  Cell<T> 只能用于 T 实现了 Copy 的情况；
  .get() 方法，返回内部值的一个拷贝
  .set() 方法，更新值。
**/
#[test]
fn cell_test() {
    let c = Cell::new(5);

    let five = c.get();
    c.set(10);
    println!("{},", c.get());
}

/*RefCell 的特点：
在不确定一个对象是否实现了 Copy 时，直接选 RefCell；
如果被包裹对象，同时被可变借用了两次，则会导致线程崩溃。所以需要用户自行判断；
RefCell 只能用于线程内部，不能跨线程；
RefCell 常常与 Rc 配合使用（都是单线程内部使用）；*/

#[test]
fn refcell_test() {
    let shared_map: Rc<RefCell<_>> = Rc::new(RefCell::new(HashMap::new()));

    //用了 RefCell 后，外面是 不可变引用 的情况，一样地可以修改被包裹的对象。
    let mut aa = shared_map.borrow_mut();

    shared_map.borrow_mut().insert("africa", 92388);
    shared_map.borrow_mut().insert("kyoto", 11837);
    shared_map.borrow_mut().insert("piccadilly", 11826);
    shared_map.borrow_mut().insert("marbles", 38);
}

#[test]
fn refcell_test01() {
    let c = RefCell::new(5);

    //不可变借用被包裹值。同时可存在多个不可变借用。
    {
        let borrowed_five = c.borrow();
    }


    let borrowed_five2 = c.borrow();

    println!("===>{}", borrowed_five2)
}

fn asdad(aa: i32) {
    println!("i32{}", aa);
}

fn asdad11(aa: String) {
    println!("str=======  {}", aa);
}


#[test]
fn refcell_test0x1() {

    let name="ssssssss".to_string();

    let hin=Rc::new(RefCell::new(name));

    let  a= RefCell::borrow(&hin).clone();
    asdad11(a);

    let  v= RefCell::borrow(&hin).clone();

    println!("final {}",v)


}
#[derive(Clone)]
struct Person;

#[derive(Clone)]
struct User{
    name:String,
    p:Person
}

impl User{

}



fn save(u:User){
    println!("user name:{}",u.name)
}

#[test]
fn refcell_test0x2() {
    let u=User{name:"yancheng".to_string(),p:Person};
    save(u.clone());
    println!("ssssss=={}",u.clone().name)
}