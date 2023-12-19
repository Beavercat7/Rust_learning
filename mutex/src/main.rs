//共享状态并发
use std::sync::Mutex;
use std::thread;
use std::sync::Arc;
//Mutex<T>的API
//在一个单线程上下文探索Mutex<T>的API
fn main() {
    /*
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    */

    //在线程间共享Mutex<T>
    /*
    let counter = Mutex::new(0);

    let mut handles = vec![];

    //程序启动了10个线程,每个线程都通过Mutex<T>来增加计数器的值
    for _ in 0..10{
        let handle = thread::spawn(move||{
            let mut num = counter.lock().unwrap();

            *num + 1;
        })
        handles.push(handle);
    }

    for handle in handles{
        handle.join().unwrap();
    }

    println!("Result: {}",*counter.lock().unwrap());
    */
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10{
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move||{
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles{
        handle.join().unwrap();
    }

    println!("Result: {}",*counter.lock().unwrap());
}

