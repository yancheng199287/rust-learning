//io（输入/输出）库引入当前作用域。io 库来自于标准库（也被称为 std）：
use std::io;

//可以查询相关文档使用，rand是一个外部依赖库，可以生成随机数
use rand::Rng;

//Ordering 也是一个枚举，不过它的成员是 Less、Greater 和 Equal。这是比较两个值时可能出现的三种结果。
use std::cmp::Ordering;

//fn 语法声明了一个新函数，() 表明没有参数，{ 作为函数体的开始。 main是函数的主入口
fn main() {
    // println! 是一个在屏幕上打印字符串的宏
    println!("Guess the number!");


    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        /* let mut guess 会引入一个叫做 guess 的可变变量。等号（=）的右边是 guess 所绑定的值，
         * 它是 String::new 的结果，这个函数会返回一个 String 的新实例。
         *  String 是一个标准库提供的字符串类型，它是 UTF-8 编码的可增长文本块
         *  ::new 那一行的 :: 语法表明 new 是 String 类型的一个 关联函数（associated function）。
         * 关联函数是针对类型实现的，在这个例子中是 String，而不是 String 的某个特定实例。
         * 一些语言中把它称为 静态方法（static method）。
         */
        let mut guess = String::new();

        //& 表示这个参数是一个 引用（reference），它允许多处代码访问同一处数据，而无需在内存中多次拷贝
        //&mut guess 来使其可变
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");//使用 Result 类型来处理潜在的错误

        //trim 方法会去除字符串开头和结尾的空白字符,parse转换
        /* let guess: u32 = guess.trim().parse()
             .expect("Please type a number!");*/

        //parse 返回一个 Result 类型，而 Result 是一个拥有 Ok 或 Err 成员的枚举。
        // 这里使用的 match 表达式，和之前处理 cmp 方法返回 Ordering 时用的一样。
        // 如果 parse 能够成功的将字符串转换为一个数字，它会返回一个包含结果数字的 Ok。这个 Ok 值与 match 第一个分支的模式相匹配，该分支对应的动作返回 Ok 值中的数字 num，最后如愿变成新创建的 guess 变量。
        // 如果 parse 不 能将字符串转换为一个数字，它会返回一个包含更多错误信息的 Err。Err 值不能匹配第一个 match 分支的 Ok(num) 模式，但是会匹配第二个分支的 Err(_) 模式：_ 是一个通配符值，本例中用来匹配所有 Err 值，不管其中有何种信息。所以程序会执行第二个分支的动作，continue 意味着进入 loop 的下一次循环，请求另一个猜测。这样程序就有效的忽略了 parse 可能遇到的所有错误！
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed:{}", guess);

        //一个 match 表达式由 分支（arms） 构成。一个分支包含一个 模式（pattern）和表达式开头的值与分支模式相匹配时应该执行的代码。
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;//退出循环
            }
        }
    }
}
