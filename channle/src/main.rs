use std::sync::mpsc;
use std::thread;
use core::time::Duration;

fn main() {
    //创建一个通道,并将其两端赋值给tx和rx
    let (tx,rx) = mpsc::channel();

    let tx1 = tx.clone();
    //将tx移动到一个新建的线程中并发送"hi"
    /*
    thread::spawn(move||{
        let val = String::from("hi");
        tx.send(val).unwrap();
    //在我们已经发送到通道中后,尝试使用val引用
        println!("val is {}",val);
    });
    //send函数获取其参数的所有权并移动这个值归接收者所有
    let received = rx.recv().unwrap();
    println!("Got: {}",received);
    */
    //发送多个消息,并在每次发送后暂停一段时间
    thread::spawn(move||{
        let vals = vec![
        String::from("hi"),
        String::from("from"),
        String::from("the"),
        String::from("thread"),];

        for val in vals{
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    //从多个生产者发送多个消息
    thread::spawn(move||{
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals{
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    for received in rx{
        println!("Got: {}",received);
    }
}
