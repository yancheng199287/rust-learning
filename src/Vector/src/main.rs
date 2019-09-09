use crate::vector::test_vector;

fn main() {
    println!("Hello, 本案例，我们学习集合相关类型!");
    test_vector();
}


pub mod vector {
    pub fn test_vector() {
        // 通过函数创建一个Vec实例
        let v: Vec<i32> = Vec::new();
        // 通过宏创建一个Vec实例，指定了内容
        let mut v = vec![1, 2, 3, 4, 5, 6, 7];


        v.push(5); //增加元素
        v.pop();//，它会移除并返回 vector 的最后一个元素
        v.remove(3);// 移除指定索引元素

        let third = &v[2];
        println!("The third element is {}", third);

        match v.get(2) {
            Some(it) => println!("The third element is {}", it),
            None => println!("There is no third element."),
        }

        /* let first = &v[0];// 获取可变引用
         v.push(6);//获取不可变引用  这里会报错  不能在相同作用域中同时存在可变和不可变引用的规则
         println!("The first element is: {}", first);
         */

        let v = vec![100, 32, 57];
        //for循环获取每一个元素的不可变引用
        for i in &v {
            println!("{}", i);
        }

        //for 循环每一个元素的可变引用以便能改变他们
        let mut v = vec![10, 20, 30];
        for i in &mut v {
            //必须使用解引用运算符（*）获取 i 中的值
            *i += 50;
            println!("{}", i);
        }
    }// <- 这里 v 离开作用域并被丢弃  当 vector 被丢弃时，所有其内容也会被丢弃
}

