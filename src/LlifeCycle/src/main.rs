/*Rust 中的每一个引用都有其 生命周期（lifetime），也就是引用保持有效的作用域。
大部分时候生命周期是隐含并可以推断的，正如大部分时候类型也是可以推断的一样。
  只有谈引用的时候才有生命周期，因为生命周期就是控制引用的有效性
*/
fn main() {
    println!("Hello, 本案例讲解生命周期与引用有效性");
    //test();
    test1();

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest1(string1.as_str(), string2);
    println!("The longest string is {}", result);


    //string1 直到外部作用域结束都是有效的
    let string1 = String::from("long string is long");

    {
        //string2 则在内部作用域中是有效的
        let string2 = String::from("xyz");
        //result 则引用了一些直到内部作用域结束都是有效的值
        let result = longest1(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }


    /*    let string1 = String::from("long string is long");
        let result;
        {
            let string2 = String::from("xyz");

            //函数返回的引用的生命周期与传入参数的生命周期中较短那个保持一致
            //string2的生命周期较短，但是result的生命周期在外面，很长，编译器不允许这样传递无效的引用的操作
            result = longest1(string1.as_str(), string2.as_str());
        }
        println!("The longest string is {}", result);

        */


    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");

    //ImportantExcerpt 离开作用域之后 novel 都不会离开作用域，所以 ImportantExcerpt 实例中的引用是有效的
    let content = ImportantExcerpt { part: first_sentence };
    println!("The first_sentence string is {}", content.part);
}


/*这个结构体有一个字段，part，它存放了一个字符串 slice，这是一个引用。
类似于泛型参数类型，必须在结构体名称后面的尖括号中声明泛型生命周期参数，以便在结构体定义中使用生命周期参数。
这个注解意味着 ImportantExcerpt 的实例不能比其 part 字段中的引用存在的更久。*/
struct ImportantExcerpt<'a> {
    part: &'a str,
}


/*//返回值的生命周期与参数完全没有关联
//出现的问题是 result 在 longest 函数的结尾将离开作用域并被清理，而我们尝试从函数返回一个 result 的引用。
// 无法指定生命周期参数来改变悬垂引用，而且 Rust 也不允许我们创建一个悬垂引用。
// 解决办法 返回 String，一个有所有权的数据类型而不是一个引用，这样函数调用者就需要负责清理这个值了。
fn longest3<'a>(x: &str, y: &str) -> &str {
    let result = String::from("really long string");
    result.as_str()
}*/

/*
// 如果只想返回指定的参数，仅仅在指定的参数标注泛型生命周期参数，而其他不需要的则不必加
fn longest2<'a>(x: &'a str, y: &str) -> &'a str {
    // 注意，因为返回值类型的生命周期是 ‘a   而参数只有x的生命周期参数是 ’a 所以只能返回x   如果返回y会提示报错
   x
}*/


//泛型生命周期参数需要声明在函数名和参数列表间的尖括号中。
// 这里我们想要告诉 Rust 关于参数中的引用和返回值之间的限制是他们都必须拥有相同的生命周期
// 函数定义指定了签名中所有的引用必须有相同的生命周期 'a
//生命周期在每次函数被调用时都可能不同。这也就是为什么我们需要手动标记生命周期。
// 函数返回的引用的生命周期应该与传入参数的生命周期中较短那个保持一致,意思是返回值的生命周期用的是 x和y中较短的生命周期，所以外部接收函数返回的变量要与最短的参数生命周期保持一致
fn longest1<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
/*
//&str是一个泛型引用，并不知道将要返回的引用是指向 x 或 y
//借用检查器自身同样也无法确定，因为它不知道 x 和 y 的生命周期是如何与返回值的生命周期相关联的
fn longest(x: &str, y: &str) -> &str {
    if x.le() > y.len() {
        x
    } else { y }
}
*/

fn test1() {
    let x = 5;
    let r;
    r = &x;  //  r x在一个作用域内，当x有效时候，r总是有效的  或者说 x的生命周期大于r，所以r结束并不会影响x
    println!("r:{}", r)
}


/*fn test() {
    let r; //外部作用域，生命周期在整个test函数有效
    {
        let x = 5;// x的生命周期较短，仅在当前代码块里
        r = &x;
    }//`x` does not live long enough  x在这里将会被销毁，而上一行代码 r获取了x的引用，r的生命周期比x长，所以编译会报错，
    // 生命周期长的引用变量不能持有比自己短的生命周期
    println!("r:{}", r)
}*/


#[test]
fn life0() {
    let a = get_str();
    println!("{}", a);

    // longest(a, "asassa");
}


/*
//返回值的生命期必须和至少一个参数相同
fn longest(x: &str, y: &str) -> &str {  //返回引用，但与参数无关，只能从内部返回。无效
    let result = String::from("really long string");
    result.as_str();  //返回后不再存在，造成无主引用。这种情况不要返回引用。
}
*/

fn get_str() -> &'static str {
    let x: &str = "Hello, world.";
    return x;
}


#[test]
fn test_life_1() {
    /*    {
            let r;                // ---------+-- 'a
            //          |
            {                     //          |
                let x = 5;        // -+-- 'b  |
                r = x;           //  |       |
            }                     // -+       |
            //          |
            println!("r: {}", r); //          |
        }



        {
            let name;
            {
                let name1 = "good study";
                name = &name1;
            }

            println!("name result:{}", name);
        }*/

    let age = "我的年龄";
    let result;
    {
        let weight = "体";
        result = longest(&age, &weight);
    }
    println!("result:{}", result);

    /*
     报错
    let age = String::from("我的年龄");

     let result;
     {
         let weight = String::from("我的体重哦");
         result = longest(&age, &weight);
         println!("小生命周期代码块中 result:{}", result);
     }
     println!("result:{}", result);*/


    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
}


#[test]
fn test_life_2() {
    let mut name = String::from("hello\taa  ss 55");
    let name1 = String::from("world");
    let a = longest(&name, &name1);
    println!("{}", a);

    let b = longest1(&mut name, &name1);
    println!("b {}", b);
    fn longest<'a>(x: &'a str, y: &str) -> &'a str {
        x
    }

    /*  fn longest1<'a>(x: &'a mut str, y: &str) -> &'a str {
         return  x.trim()
      }*/

    /*  fn longest1<'a>(x: &str, y: &str) -> String {
          return String::from("really long string");
      }
  */

    /* fn longest1<'a>(x: &str, y: &str) -> &'a str {
         let result = String::from("really long string");
         result.as_str()
     }
 */
    /* let result = String::from("really long string");
     result.as_str()*/
}


#[test]
fn test_life_3() {

    let r;

    {
        let x=5;
        r=x;

    }
    print!("{}", r)

}