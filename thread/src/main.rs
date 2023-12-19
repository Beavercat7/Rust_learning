//使用spawn创建新线程
use std::thread;
use std::time::Duration;
//创建一个打印某些内容的新线程，但是主线程打印其他内容
fn main() {
    // let handle = thread::spawn(||{
    //     for i in 1..10{
    //         println!("hi number {} from the spawned thread!",i);
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });
    // handle.join().unwrap();
    // for i in 1..5{
    //     println!("hi number {} from the main thread!",i);
    //     thread::sleep(Duration::from_millis(1));
    // }

    //使用join等待所有线程结束
    //从thread::spawn保存一个JoinHandle以确保该线程能够运行结束
   // handle.join().unwrap();
    let v = vec![1,2,3];

    //Rust默认引用v,但是主线程没有办法保证引用的有效性
    //使用move关键字强制获取它使用的值的所有权
    let handle = thread::spawn(move||{
        println!("Here's a vector: {:?}",v);
    });

    handle.join().unwrap();
}

