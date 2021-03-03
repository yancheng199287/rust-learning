# rust通用知识指南

> 本文档是快速入门，总结rust基本语法，特性，注意事项，帮助新手快速入门，帮助很久没有使用rust的快速复习上手



# 官方文档

doc
https://doc.rust-lang.org/book/

example
https://doc.rust-lang.org/stable/rust-by-example/index.html



# 基本语法

## 变量与常量

标识符：变量、函数、结构体等等，所有这些都需要名称，它们可以是任意非空的 ASCII 字符串，推荐命名的方式是多个单词用下划线分隔

变量与可变性：
变量的默认声明是不可变的，    let x = 5; 如果 执行  x=6;将会导致编译报错，如果需要修改改变量，必须声明为可变变量 加上关键词 mut，如  let mut x = 5; 

常量与不可变变量

1. 常量声明需要使用 const关键词，而不是 let，如：const MAX_POINTS: u32 = 100_000;
2. 常量可以在任何作用域中声明，包括全局作用域，这在一个值需要被很多部分的代码用到时很有用
3. 常量只能被设置为常量表达式，而不能是函数调用的结果，或任何其他只能在运行时计算出的值


隐藏的概念： 隐藏可以使用let关键字重新定义一个上面存在的变量名称，上面的变量就会被隐藏，无法再使用了

隐藏与可变变量的区别

1. 隐藏必须使用let关键字，然后可以使用上面的变量进行改变，赋值给新的同名变量，并且数据类型可以不一致
   let spaces = "   "; //字符串类型
   let spaces = spaces.len();//复用上面的变量，得到一个整型类型，赋值给一个同名变量

2. 隐藏与将变量标记为 mut 是有区别的。当不小心尝试对变量重新赋值时，如果没有使用 let 关键字，就会导致编译时错误   



## 数据类型

在 Rust 中，每一个值都属于某一个 数据类型（data type），这告诉 Rust 它被指定为何种数据，以便明确数据处理方式。我们将看到两类数据类型子集：标量（scalar）和复合（compound）。
Rust 是 静态类型（statically typed）语言，也就是说在编译时就必须知道所有变量的类型。根据值及其使用方式，编译器通常可以推断出我们想要用的类型。当多种类型均有可能时，比如第二章的 “比较猜测的数字和秘密数字” 使用 parse 将 String 转换为数字时，必须增加类型注解

标量（scalar）类型代表一个单独的值
标量类型：四种基本的标量类型：整型、浮点型、布尔类型和字符类型。

复合类型（Compound types）可以将多个值组合成一个类型。Rust 有两个原生的复合类型：元组（tuple）和数组（array）



## 函数

main 函数，它是很多程序的入口点。你也见过 fn 关键字，它用来声明新函数。

//具有返回值类型，最后一行，可以不用return关键词，返回值类型用箭头指向

```rust
fn plus_one(x: i32) -> i32 {
    x + 1
}
```





## 控制流

if表达式：

```rust
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
```

   使用let关键字 赋值 简写

```rust
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };
```


loop循环：

```rust
let mut counter = 0;
let result = loop {
    counter += 1;
    if counter == 10 {
        break counter * 2;
    }
};
```

while条件循环：

```rust
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number = number - 1;
    }

```

for遍历：


```rust
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }    

    for number in (1..4).rev() {
        println!("{}!", number);
    }

```



## 理解所有权

> 所有权（系统）是 Rust 最独特的功能，其令 Rust 无需垃圾回收（garbage collector）即可保障内存安全
>
> 变量的所有权就是变量真身，有其作用域限制，可以有很多分身引用

所有权规则

1. Rust 中的每一个值都有一个被称为其 所有者（owner）的变量。
2. 值有且只有一个所有者。
3. 当所有者（变量）离开作用域，这个值将被丢弃。

```rust
{                    
   // s 在这里无效, 它尚未声明
   let s = "hello";   // 从此处起，s 是有效的
   // 这里可以使用s
}                      // 此作用域已结束，s 不再有效

```



## 变量与数据交互的方式

（一）：移动

```rust
let x = 5;
let y = x;
```

此处 x将数据拷贝到y， x和y变量均是有效的，因为存放在栈上的数据，拷贝速度非常快

此处 x将数据拷贝到y， x和y变量均是有效的，因为存放在栈上的数据，拷贝速度非常快

```
let s1 = String::from("hello");//在堆上分配内存存放字符串，但是s1指针是在栈上的
let s2 = s1;//从栈上拷贝了s1它的指针、长度和容量。我们并没有复制指针指向的堆上数据，这里拷贝s1的指针后，s1立即失效释放内存，后续不能在使用了，防止二次释放（double free）的错误   string类型不能copy  只能对引用指针copy  
```

（二）：克隆

```rust
let s1 = String::from("hello");
let s2 = s1.clone();// 需要深度复制 String 中堆上的数据，而不仅仅是栈上的数据，可以使用一个叫做 clone 的通用函数，可能相当消耗资源
```



注意：一个类型拥有 Copy trait，一个旧的变量在将其赋值给其他变量后仍然可用，以上是自动实现了 Copy trait的数据类型，就像 上面移动 那样
所有整数类型，比如 u32。
布尔类型，bool，它的值是 true 和 false。
所有浮点数类型，比如 f64。
字符类型，char。
元组，当且仅当其包含的类型也都是 Copy 的时候。比如，(i32, i32) 是 Copy 的，但 (i32, String) 就不是



## 所有权与函数

将值传递给函数在语义上与给变量赋值相似。向函数传递值可能会移动或者复制，就像赋值语句一样

```rust
fn main() {
    let s = String::from("hello");  // s 进入作用域

    takes_ownership(s);             // s 的值移动到函数里  这个时候所有权交出去了，rust叫 move了 因为string非copy
                                    // ... 所以到这里不再有效
    
    //当尝试在调用 takes_ownership 后使用 s 时，Rust 会抛出一个编译时错误 move了
    // s.length();                                
    
    let x = 5;                      // x 进入作用域
    
    makes_copy(x);                  // x 应该移动函数里，
                                    // 但 i32 是 Copy 的，所以在后面可继续使用 x

} // 这里, x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
  // 所以不会有特殊操作

fn takes_ownership(some_string: String) { // some_string 进入作用域
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

fn makes_copy(some_integer: i32) { // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。不会有特殊操作

```

返回值与作用域
返回值也可以转移所有权


```rust
fn main() {
let s1 = gives_ownership();         // gives_ownership 将返回值 移给 s1

let s2 = String::from("hello");     // s2 进入作用域

let s3 = takes_and_gives_back(s2);  // s2 被移动到  takes_and_gives_back 中, 它也将返回值移给 s3

} // 这里, s3 移出作用域并被丢弃。s2 也移出作用域，但已被移走，
  // 所以什么也不会发生。s1 移出作用域并被丢弃


fn gives_ownership() -> String {             // gives_ownership 将返回值移动给调用它的函数

    let some_string = String::from("hello"); // some_string 进入作用域
    
    some_string                              // 返回 some_string 并移出给调用的函数
}

// takes_and_gives_back 将传入字符串并返回该值
fn takes_and_gives_back(a_string: String) -> String { // a_string 进入作用域

    a_string  // 返回 a_string 并移出给调用的函数
}

```



变量的所有权总是遵循相同的模式：将值赋给另一个变量时移动它。当持有堆中数据值的变量离开作用域时，其值将通过 drop 被清理掉，除非数据被移动为另一个变量所有



引用与借用
我们将获取引用作为函数参数称为 借用（borrowing）。正如现实生活中，如果一个人拥有某样东西，你可以从他那里借来。当你使用完毕，必须还回去。



```rust
let s1 = String::from("hello");
let len = calculate_length(&s1);
fn calculate_length(s: &String) -> usize { // s 是对 String 的引用
    s.len()
} // 这里，s 离开了作用域。但因为它并不拥有引用值的所有权，
  // 所以什么也不会发生
变量 s 有效的作用域与函数参数的作用域一样，不过当引用离开作用域后并不丢弃它指向的数据，因为我们没有所有权。当函数使用引用而不是实际值作为参数，无需返回值来交还所有权，因为就不曾拥有所有权。


//如果要引用的值进行修改的话，需要指定mut关键词，首先，必须将 some_string 改为 mut。然后必须创建一个可变引用 &mut some_string 和接受一个可变引用 some_string: &mut String。
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

//注意：可变引用有一个很大的限制：在特定作用域中的特定数据有且只有一个可变引用。这些代码会失败：
let mut s = String::from("hello");
let r1 = &mut s;
let r2 = &mut s;//不能拥有多个可变引用  防止数据竞争
println!("{}, {}", r1, r2);


```



悬垂引用
悬垂指针是其指向的内存可能已经被分配给其它持有者或者已经被drop掉了

```rust
fn dangle() -> &String { // dangle 返回一个字符串的引用

    let s = String::from("hello"); // s 是一个新字符串
    
    &s // 返回字符串 s 的引用
} // 这里 s 离开作用域并被丢弃。其内存被释放。
  // 危险！
 // 因为 s 是在 dangle 函数内创建的，当 dangle 的代码执行完毕后，s 将被释放。不过我们尝试返回它的引用。这意味着这个引用会指向一个无效的 String，这可不对！Rust 不会允许我们这么做。

// 解决方法是直接返回 String
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}  

```

引用的规则
**在给定的作用域中，要么只能有一个可变引用，要么只能有多个不可变引用，引用必须总是有效。**



## Slice 类型

除了引用没有所有权，另一个没有所有权的数据类型是 slice。slice 允许你引用集合中一段连续的元素序列，而不用引用整个集合。

```rust
//字符串 slice
let s = String::from("hello world");

let hello = &s[0..5];
let world = &s[6..11];

let hello1 = &s[0..=4];
let world2 = &s[6..=10];
//这类似于引用整个 String 不过带有额外的 [0..5] 部分。它不是对整个 String 的引用，而是对部分 String 的引用。start..end 语法代表一个以 start 开头并一直持续到但不包含 end 的 range。如果需要包含 end，可以使用 ..= 而不是 .

//字符串字面值就是 slice
let s = "Hello, world!";//这里 s 的类型是 &str：它是一个指向二进制程序特定位置的 slice。这也就是为什么字符串字面值是不可变的；&str 是一个不可变引用


```



## 关于智能指针

**要么只能有一个可变引用（用于修改，只能一个，防止数据竞争），要么只能有多个不可变引用（多个读取没问题）,引用必须总是有效（引用所指向的值变量没有被清理掉）**

### Box

> Box 在堆上分配内存，允许存储编译未知大小的类型（例如递归类型），同普通数据所有权一样，离开作用域即释放资源。
> 应用场景：数据想存在堆中，编译时不可知大小（通过Box指向未知大小的引用）

```rust
// 没有什么特别的功能，跟其他普通数据类型的所有权一致，离开作用域就释放
let b = Box::new(5);
let b1 = b.borrow() + 1;
```

### Rc

> Rc 引用计数，启用多所有权，通过clone供其他程序多个地方使用读取，clone一次增加引用计数+1，离开作用域自动-1
> 应用场景：变量想在多处使用，不想因为作用域move掉而失效

```rust
let s1 = String::from("Hello");
// s1进入之后 move
let ab = Box::new(s1);
// s1已经move，编译失败
println!("{}", s1)

// 通过Rc改造
let s1 = Rc::new(String::from("Hello"));
// cloney一次 s1的引用+1
let ab = Box::new(s1.clone());
// 正常使用，因为引用计数不为0，意味着多所有权
println!("当前引用计数：{}，其值是：{}", Rc::strong_count(&s1), s1);
```



### Cell 

> Cell 针对已经实现Copy特性的类型（必须的），通过get方法自动克隆拷贝，通过set方法重新赋值，免去手动去克隆

```rust
// 包裹一个实现了copy的类型，get的时候是copy，set的时候直接替换
let age = Cell::new(152);
test_cell_2(age.get());
age.set(200);
println!("test_cell_1:{}", age.get())
```



### RefCell

RefCell当创建不可变和可变引用时，我们分别使用 & 和 &mut 语法。
对于 RefCell<T> 来说，则是 borrow 和 borrow_mut 方法
对于是否实现Copy的trait特性不在意，但是在运行时候进行检查，可变引用只能借用一次，再借用会报异常。



## 生命周期

```rust
只有谈引用的时候才有生命周期，因为生命周期就是控制引用的有效性
生命周期其实是一个注释，绑定了关联的变量，一般用 `开头  例如 `a    `b 
```



生命周期长的引用不能被与之较短的生命周期引用赋值，只能比它自己更长的生命周期赋值

因为短生命周期挂掉了，长生命周期的变量就指向了一个无效引用，也叫悬垂引用

### 

### 借用检查器

```rust
   {
        let r;
        {
            let x = 5;
            // 这里报错， 因为r的生命周期更长，  x的生命周期更短，不能把短的给长的！！  注意这里使用了 &x ，如果使用了x也是没问题（此时跟生命周期没关系，因为生命周期只跟引用有关系，跟本身的变量值没关系）
            r = &x;
        }
        println!("r: {}", r);
    }

// 修复上面的错误
  {
      let x = 5;
      let  r = &x;  // 此时 x的生命周期大于r的生命周期， 是没问题的
       println!("r: {}", r);
    }



```



### 函数的通用生命周期



函数的返回值是多个参数引用中的任意一个则必须注明生命周期注释

```rust

   let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

// 两个参数引用， 最终返回其中一个，编译器不知得二个参数的生命周期范围是怎样的，通过if判定，都可以被执行返回
// 如果返回了较小的生命周期，最终结果值却在更大的生命周期作用域被使用了，就产生问题了
// 必须指定生命周期注释  两个参数既可以一样，也可以不一样
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 修正之后
//  两个参数指向了相同的生命周期参数，返回值也是相同的
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```







不同生命周期的参数引用，返回的生命周期以最小的哪个为准

```rust
// 运行正常    
let age = "我的年龄";
    let result;
    {
        let weight = "体";
        // 字符串比较特殊，这里实际不是传的引用，又好像是，但确实没问题
        result = longest(&age, &weight);
    }
    println!("result:{}", result);


//-------------------
// 运行报错 运行报错 运行报错
   let age =String::from( "我的年龄");

      let result;
      {
          let weight = String::from("体");
          // 这里一定是引用  会触发生命周期判定    函数如果多个参数引用，会返回最小的生命周期（weight生命周期），但是这里result是个大的，会报错
          result = longest(&age, &weight);
      }
    //  因为这个值使用了大的生命周期，不允许小的生命周期赋值， 如果去掉这行 不会报错 删除不使用则不影响
      println!("result:{}", result);


fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

```



函数返回值的生命周期可以指定某个参数的引用生命周期

指定生存期参数的方式取决于函数的功能

函数如果返回引用类型，则必须与参数的某个生命周期保持一致，否则编译失败，因为无法确定

函数如果返回引用类型，则必须是参数中的某个生命周期注释，函数想返回引用类型只能是参数本身，不能是函数内部变量的引用

如果函数想返回函数内部创建的变量，之间返回变量，不要返回引用

```rust
// 没有问题， 函数的返回值始终与x的生命周期保持一致
fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}


// 报错，因为函数返回了一个内部函数中的变量引用，与参数的生命周期不一致，并且函数结束之后，result就drop掉了，返回的也是一个无效引用
fn longest1<'a>(x: &str, y: &str) -> &'a str {
        let result = String::from("really long string");
        result.as_str()
   }


// 编译失败， 虽然函数返回值没有生命周期参数，但是返回的是函数内部的引用，函数结束之后就drop掉，无效的引用
  fn longest1<'a>(x: &str, y: &str) -> &str {
        let result = String::from("really long string");
        result.as_str()
    }

// 编译成功， 返回的是一个变量值，不是引用哦
   fn longest1<'a>(x: &str, y: &str) -> String {
        return String::from("really long string");
    }
```





### 方法生命周期注解参数自动推导三条规则

1. 引用的每个参数都具有其自己的生命周期参数

   `fn foo<'a>(x: &'a i32)`; 带有两个参数的函数获得两个单独的寿命参数：`fn foo<'a, 'b>(x: &'a i32, y: &'b i32)`; 

2. 如果仅存在一个输入生命周期参数，则将生命周期分配给输出生命周期参数

   `fn foo<'a>(x: &'a i32) -> &'a i32`

3. 如果有多个输入生命周期参数，但是其中一个是，`&self`或者`&mut self`因为这是一种方法，所以将的生命周期`self` 分配给所有输出参数

   ```rust
   impl<'a> ImportantExcerpt<'a> {
       fn level(&self) -> i32 {
           3
       }
   }
   
   impl<'a> ImportantExcerpt<'a> {
       fn announce_and_return_part(&self, announcement: &str) -> &str {
           println!("Attention please: {}", announcement);
           self.part
       }
   }
   ```





### 静态生命周期

该字符串的文本直接存储在程序的二进制文件中，该二进制文件始终可用。类似静态变量，存在于整个应用中。

如果在编写的时候很苦恼生命周期长短问题，可以临时调试使用，但是确保你一定要这么做？请在尝试其他方法无法解决的时候使用

```rust
let s: &'static str = "I have a static lifetime.";
```



### 泛型类型参数，特质界限和寿命

```rust
use std::fmt::Display;

// 生命周期期是一种通用类型，因此e参数`'a`和泛型类型参数 的声明`T`在函数名称后的尖括号内的同一列表
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,  // 泛型类型的额外参数
) -> &'a str
where    // 该`where`子句指定的特征的任何类型填充
    T: Display, // 该参数`T`可以由实现`Display` 
{
    println!("Announcement! {}", ann);  // 对实现了Display特性的T类型参数进行打印
   // 返回两个字符串切片中的较长者 
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

