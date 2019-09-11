use std::thread;
use std::time::Duration;


/*闭包不要求像 fn 函数那样在参数和返回值上注明类型。函数中需要类型注解是因为他们是暴露给用户的显式接口的一部分。
严格的定义这些接口对于保证所有人都认同函数使用和返回值的类型来说是很重要的。
但是闭包并不用于这样暴露在外的接口：他们储存在变量中并被使用，不用命名他们或暴露给库的用户调用。
*/

fn main() {
    println!("Hello, 本案例演示了闭包!");
}

fn closure_capture_env1() {
    let x = vec![1, 2, 3];

    //强制闭包获取其使用的环境值的所有权，可以在参数列表前使用 move 关键字
    let equal_to_x = move |z| z == x;

    // 由于上面的闭包使用move关键字 将x的所有权移动走了，x在这里失效了，不能使用
  //  println!("can't use x here: {:?}", x);

    let y = vec![1, 2, 3];

    assert!(equal_to_x(y));
}

fn closure_capture_env() {
    let x = 4;

    //闭包中使用了外部的变量x，这个是允许的，即便 x 并不是 equal_to_x 的一个参数，equal_to_x 闭包也被允许使用变量 x，因为它与 equal_to_x 定义于相同的作用域。
    //函数则不能做到同样的事,如果函数这样做，会报错

    //当闭包从环境中捕获一个值，闭包会在闭包体中储存这个值以供使用。这会使用内存并产生额外的开销，在更一般的场景中，当我们不需要闭包来捕获环境时，我们不希望产生这些开销。
    // 因为函数从未允许捕获环境，定义和使用函数也就从不会有这些额外开销。
    let equal_to_x = |z| z == x;
    let y = 4;
    assert!(equal_to_x(y));
}


fn generate_workout3(intensity: u32, random_number: u32) {
    /***
    *我们保存一个新的 Cacher 实例来存放闭包。接着，在每一个需要结果的地方，调用 Cacher 实例的 value 方法。
    *可以调用 value 方法任意多次，或者一次也不调用，而慢计算最多只会运行一次。
    ***/
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(3));
        num
    });

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_result.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}


/*结构体 Cacher 有一个泛型 T 的字段 calculation。T 的 trait bound 指定了 T 是一个使用 Fn 的闭包。
任何我们希望储存到 Cacher 实例的 calculation 字段的闭包必须有一个 u32 参数（由 Fn 之后的括号的内容指定）并必须返回一个 u32
*/
struct Cacher<T> where T: Fn(u32) -> u32 {
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T> where T: Fn(u32) -> u32 {
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            // 如果 value有值 直接返回
            Some(v) => v,
            // 如果 value没有值 则进行闭包计算
            None => {
                // self.calculation是闭包函数，arg是该闭包的参数
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}


fn multipart_form_closure() {
    fn add_one_v1(x: u32) -> u32 { x + 1 }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x: u32| { x + 1 };

    //这个闭包没有指定类似参数和返回类型参数，但是可以根据下面使用add_one_v4传递的参数推断类型，如果该闭包没有使用 则会报错，因为无法推断类型
    let add_one_v4 = |x| { x };
    let o = add_one_v4(52);
}


fn generate_workout2(intensity: u32, random_number: u32) {


    // 注意这个 let 语句意味着 expensive_closure 包含一个匿名函数的 定义，不是调用匿名函数的 返回值。
    // 回忆一下使用闭包的原因是我们需要在一个位置定义代码，储存代码，并在之后的位置实际调用它；
    // let 后 紧跟 变量名 =  竖号|是闭包参数  多个 用逗号分割开来 函数体用大括号包括跟函数没区别
    let expensive_closure = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_closure(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_closure(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_closure(intensity)
            );
        }
    }
}

fn generate_workout1(intensity: u32, random_number: u32) {
    //将重复的 simulated_expensive_calculation 函数调用提取到一个变量中
    let expensive_result = simulated_expensive_calculation(intensity);

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result
        );
        println!(
            "Next, do {} situps!",
            expensive_result
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result
            );
        }
    }
}


fn generate_workout(intensity: u32, random_number: u32) {
    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            simulated_expensive_calculation(intensity)
        );
        println!(
            "Next, do {} situps!",
            simulated_expensive_calculation(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                simulated_expensive_calculation(intensity)
            );
        }
    }
}


fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}