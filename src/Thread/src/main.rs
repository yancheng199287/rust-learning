use std::thread;
use std::time::Duration;
use std::sync::mpsc;


fn main() {
    println!("Hello, world!");
    // thread1();
    //thread2();
    thread3();
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