use std::env;
use std::fs;

//重构改进模块化和错误处理
//提取参数解析器
//从main中提取出parse_config函数
struct Config{
   query:String,
   filename:String,
}
//从new中返回Result而不是调用panic!
impl Config{
   fn new(args:&[String])->Result<Config,&'static str>{
      if args.len() < 3{
         //panic!("not enough arguments");
         return Err("not enough arguments");
      }
      let query = args[1].clone();
      let filename = args[2].clone();
      Config{query,filename}
   }
}
fn main() {
  let args:Vec<String> = env::args().collect();
  let config = Config::new(&args);

  println!("Searching for {}",config.query);
  println!("In file {}",config.filename);

  //修复错误处理
  //改善错误信息

}
