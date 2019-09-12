use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::sync::{Mutex, Arc};


fn main() {
    println!("Hello, world!");
    // thread1();
    //thread2();
    // thread3();
    // thread4();
    // thread5();
    //thread6();
    // thread7();
     thread8();
}

//原子引用计数 Arc<T>
fn thread8() {
    //所幸 Arc<T> 正是 这么一个类似 Rc<T> 并可以安全的用于并发环境的类型。字母 “a” 代表 原子性（atomic），所以这是一个原子引用计数（
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());

}
//

//编译会报错，因为counter不能用在多线程中，这是不安全的
/*fn thread7() {
    let counter = Mutex::new(0);
    let mut handles = vec![];
    for _ in 0..10 {
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap());
}*/


//
fn thread6() {
    //使用关联函数 new 来创建一个 Mutex<T> 互斥器
    let m = Mutex::new(5);

    {
        //使用 lock 方法获取锁，以访问互斥器中的数据。这个调用会阻塞当前线程，直到我们拥有锁为止。
        //如果另一个线程拥有锁，并且那个线程 panic 了，则 lock 调用会失败。
        // 在这种情况下，没人能够再获取锁，所以这里选择 unwrap 并在遇到这种情况时使线程 panic。

        //一旦获取了锁，就可以将返回值（在这里是num）视为一个其内部数据的可变引用了。
        // 类型系统确保了我们在使用 m 中的值之前获取锁：Mutex<i32> 并不是一个 i32，所以 必须 获取锁才能使用这个 i32 值。
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}


//通过克隆发送者来创建多个生产者
fn thread5() {
    let (tx, rx) = mpsc::channel();

    //通道的发送端调用了 clone 方法。这会给我们一个可以传递给第一个新建线程的发送端句柄。我们会将原始的通道发送端传递给第二个新建线程。
    // 这样就会有两个线程，每个线程将向通道的接收端发送不同的消息
    let tx1 = mpsc::Sender::clone(&tx);

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}


//发送多个值并观察接收者的等待
fn thread4() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    //将 rx 当作一个迭代器。对于每一个接收到的值，我们将其打印出来。当通道被关闭时，迭代器也将结束
    //因为在主线程中并没有任何暂停或位于 for 循环中用于等待的代码，所以可以说主线程是在等待从新建线程中接收值
    for received in rx {
        println!("Got: {}", received);
    }
}

//使用消息传递在线程间传送数据
fn thread3() {
    //这里使用 mpsc::channel 函数创建一个新的通道；mpsc 是 多个生产者，单个消费者（multiple producer, single consumer）的缩写。
    //mpsc::channel 函数返回一个元组：第一个元素是发送端，而第二个元素是接收端。由于历史原因，tx 和 rx 通常作为 发送者（transmitter）和 接收者（receiver）的缩写

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    /* 通道的接收端有两个有用的方法：recv 和 try_recv。这里，我们使用了 recv，它是 receive 的缩写。这个方法会阻塞主线程执行直到从通道中接收一个值。
     一旦发送了一个值，recv 会在一个 Result<T, E> 中返回它。当通道发送端关闭，recv 会返回一个错误表明不会再有新的值到来了。
     try_recv 不会阻塞，相反它立刻返回一个 Result<T, E>：Ok 值包含可用的信息，而 Err 值代表此时没有任何消息。如果线程在等待消息过程中还有其他工作时使用 try_recv 很有用：可以编写一个循环来频繁调用 try_recv，再有可用消息时进行处理，其余时候则处理一会其他工作直到再次检查。
     处于简单的考虑，这个例子使用了 recv；主线程中除了等待消息之外没有任何其他工作，所以阻塞主线程是合适的。
     */
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}


//线程与 move 闭包
fn thread2() {
    let v = vec![1, 2, 3];

    //通过在闭包之前增加 move 关键字，我们强制闭包获取其环境周围使用值的所有权， 环境周围指代在上面的可访问的作用域的值
    //这里使用move 将会把 v 移动进闭包的环境中，如此将不能在主线程中对其调用v了，因为在闭包中会被销毁了
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}


//使用 join 等待所有线程结束
fn thread1() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    //主线程阻塞，执行完成上面的线程才继续向下执行
    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    //在这里主线程等待上面线程完成，打印会交替
    //JoinHandle 并调用 join 来确保新建线程在 main 退出前结束运行：
    //通过调用 handle 的 join 会阻塞当前线程直到 handle 所代表的线程结束。阻塞（Blocking） 线程意味着阻止该线程执行工作或退出
    // handle.join().unwrap();
}