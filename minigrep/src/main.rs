use std::env;
use std::fs;
use std::process;
use std::error::Error;
//重构改进模块化和错误处理
//提取参数解析器
//从main中提取出parse_config函数
struct Config{
   query:String,
   filename:String,
}
//从new中返回Result而不是调用panic!
//&'static str是字符串字面值的类型,也是目前的错误信息
//从main提取逻辑
impl Config{
   fn new(args:&[String])->Result<Config,&'static str>{
      if args.len() < 3{
         //panic!("not enough arguments");
         return Err("not enough arguments");
      }
      let query = args[1].clone();
      let filename = args[2].clone();
      Ok(Config{query,filename})
   }
}
//现在run函数包含了main中从读取文件开始的剩余的所有逻辑
//run函数获取一个Config实例作为参数。
//从run函数中返回错误
//修改run函数返回Result
fn run(config:Config)->Result<(),Box<dyn Error>>{
   let contents = fs::read_to_string(config.filename)?;
   println!("With text:\n{}",contents);
   Ok(())
}

fn main() {
  let args:Vec<String> = env::args().collect();
  //Config::new调用并处理错误
  //如果新建Config失败则使用错误码退出
  let config = Config::new(&args).unwrap_or_else(|err|{
   println!("Problem parsing arguments: {}",err);
   process::exit(1);
  });

  println!("Searching for {}",config.query);
  println!("In file {}",config.filename);

  run(config);
  //处理main中run返回的错误
  //我们将检查错误
  if let Err(e) = run(config){
   //打印错误并退出
   println!("Application error: {}",e);
   process::exit(1);
  }

  
}
