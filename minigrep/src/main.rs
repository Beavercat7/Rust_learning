use std::env;

//为了确保minigrep能够获取传递给它的命令行参数的值
//我们需要一个Rust标准库提供的函数,也就是std::env::args
//将命令行参数收集到一个vector并打印出来
//读取参数值
fn main() {
   let args:Vec<String> = env::args().collect();
   
   let query = &args[1];
   let filename = &args[2];

   println!("Searching for {}",query);
   println!("In file {}",filename);
}
