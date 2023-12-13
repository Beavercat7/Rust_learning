use std::env;

//读取参数值
fn main() {
   let args:Vec<String> = env::args().collect();
   println!("{:?}",args);
}
