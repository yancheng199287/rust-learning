// 地球
#[derive(Debug)]
struct Earth {
    location: String,
}

// 恐龙
#[derive(Debug)]
struct Dinosaur<'a> {
    location: &'a Earth,
    name: String,
}


fn main() {
    let new_york = Earth {
        location: "New York, NY 01".to_string(),
    };

    let t_rex = Dinosaur {
        location: &new_york,
        name: "T Rex".to_string(),
    };


    println!("{:?}", t_rex);
}

/*关于 Ownership 的三条规则，以便分析：
1. 所有的资源只能有一个主人（owner）。
2. 其它人可以租借这个资源。
    同时可以有多个不可变引用(&T)。
    同时只可以有一个可变引用(&mut T)。
3. 但当这个资源被借走时，主人不允许释放或修改该资源。*/


#[test]
fn ownership01() {
    // 基本类型因为自动实现了Copy trait ，在移动的时候，a不受影响
    let a = 100;
    let b = a;
    println!("a={}", a);

    //&T 引用不可变类型因为自动实现了Copy trait，在移动的时候，c不受影响
    let c = "hello world";
    let d = c;
    println!("c={}", c);


    // &mut T 是没有实现Copy trait  但是可以通过显示指定数据类型赋值给其他变量
    let mut m = 100;
    let n = &mut m;
    //不会报错 n不会失效
    let y: &mut i32 = n;
    // 会报错，因为 &mut没有实现Copy trait   但是指定了y的数据类型 &mut i32  则可以编译通过 原因不详
    //let y= n;
    println!("n={}", n);


    //1. 所有权只有一个主人。当然你可以把书“给”其它人，所有权就归其它人，原主人就失效被销毁了,String是没有实现Copy  如果实现了 Copy 则不受影响，但是Copy是在堆上操作耗费内存
    let e = String::from("book"); // "book" 归 a 所有
    let f = &e;                    // a 将 "book" 转让给 b
    println!("e = {}", e);        // 出错，a 已经无权使用资源
}




/*
fn sas() {
    let a = String::from("book");
    {
        let b = a;            // a 将 "book" 转让给 b
    }                         // b 死了，却没有将 "book" 还给 a
    println!("a = '{}'", a);  // 出错，"book" 不在 a 手上。
}
*/


