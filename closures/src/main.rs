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
    if intensity < 25{
        println!("Today, do {} pushups!",simulated_expensive_calculation(intensity));
    println!("Next, do {} situps!",simulated_expensive_calculation(intensity));
    }
    else{
        if random_number == 3{
            println!("Take a break today! Remember to stay hydrated!");
        }
        else {
            println!("Today, run for {} minutes",simulated_expensive_calculation(intensity));
        }
    } 
}
fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;
    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );
}
