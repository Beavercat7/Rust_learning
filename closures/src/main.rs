//Rust的闭包是可以保存进变量或作为参数传递给其他函数的匿名函数
//使用闭包创建行为的抽象
//闭包:可以捕获环境的匿名函数

use std::thread;
use std::time::Duration;
//一个用来代替假定计算的函数,它大约会执行两分钟
fn simulated_expensive_calculation(intensity:u32)->u32{
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}
fn generate_workout(intensity:u32,simulated_random_number:u32){
    //定义一个闭包并储存到变量expensive_closure中
    // let expensive_closure = |num|{
    //     println!("calculating slowly...");
    //     thread::sleep(Duration::from_secs(2));
    //     num
    // };
    let mut expensive_result = Cacher::new(|num|{
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    //调用定义的expensive_closure
    if intensity < 25{
        println!("Today, do {} pushups!",expensive_result.value(intensity));
    println!("Next, do {} situps!",expensive_result.value(intensity));
    }
    else{
        if random_number == 3{
            println!("Take a break today! Remember to stay hydrated!");
        }
        else {
            println!("Today, run for {} minutes",expensive_result.value(intensity));
        }
    } 
    //在generate_workout函数中利用Cacher结构体来抽象出缓存逻辑
}
struct Cacher<T>{
    where T:Fn(u32) -> u32
    {
        calcuation:T,
        value:Option<u32>,
    }
}
impl<T>Cacher<T>
   where T:Fn(u32)->u32
{
   fn new (calcuation:T)->Cacher<T>{
    Cacher{
        calcuation,
        value:None,
    }
   }

   fn value(&mut self,arg:u32)->u32{
    match self.value{
        Some(v) => v,
        None => {
            let v = (self.calcuation)(arg);
            self.value = Some(v);
            v
        },
    }
   }
}
fn iterator_demonstration(){
    let v1 = vec![1,2,3];
    
    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(),Some(&1));
    
}
fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;
    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );
    //闭包定义会为每个参数和返回值推断一个具体类型
    //尝试调用一个被推断为两个不同类型的闭包
    //第一次使用 String 值调用 example_closure 时，
    //编译器推断 x 和此闭包返回值的类型为 String。
    //接着这些类型被锁定进闭包 example_closure 中，
    //如果尝试对同一闭包使用不同类型则会得到类型错误。
    let example_closure = |x|x;
    let s = example_closure(String::from("hello"));
    let n = example_closure(5);

    //使用带有泛型和Fn trait的闭包
    //创建一个存放闭包和调用闭包结构的结构体
    //该结构体只会在需要结果时执行闭包，并会缓存结果值，这样余下的代码就不必再负责保存结果并可以复用该值。你可能见过这种模式被称 memoization 或 lazy evaluation （惰性求值）。

    //闭包还有一个功能(捕获环境):可以捕获花环境并访问其被定义的作用域的变量
    let x = 4;
    let equal_to_x = |z|z ==x;
    let y = 4;
    assert!(equal_to_x(y));
    //三种获取参数的方式:获取所有权,可变借用,不可变借用
    //对应三个fn trait

    //使用迭代器处理元素序列beavercat7@ep.plctlab.org

    let v1 = vec![1,2,3];

    let v1_iter = v1.iter();
    //在一个for循环中使用迭代器
    for val in v1_iter{
        println!("Got: {}",val);
    }
}
///消费迭代器的方法
//使用迭代器处理元素序列

