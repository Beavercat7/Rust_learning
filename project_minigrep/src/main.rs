use std::process;
use std::env;
use project_minigrep::Config;
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

  //处理main中run返回的错误
  //我们将检查错误
  if let Err(e) = project_minigrep::run(config){
   //打印错误并退出
   println!("Application error: {}",e);
   process::exit(1);
  }

  //将代码拆分到库crate
}
