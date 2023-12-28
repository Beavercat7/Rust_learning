use std::process;
use std::env;
use project_minigrep::Config;
fn main() {
  //let args:Vec<String> = env::args().collect();
  //Config::new调用并处理错误
  //如果新建Config失败则使用错误码退出
  //使用eprintln!将错误信息写入标准错误而不是标准输出
  let config = Config::new(env::args()).unwrap_or_else(|err|{
   eprintln!("Problem parsing arguments: {}",err);
   process::exit(1);
  });

  println!("Searching for {}",config.query);
  println!("In file {}",config.filename);

  //处理main中run返回的错误
  //我们将检查错误
  if let Err(e) = project_minigrep::run(config){
   //打印错误并退出
   eprintln!("Application error: {}",e);
   process::exit(1);
  }

  //将代码拆分到库crate

  //现在我们将逻辑提取到了 src/lib.rs 并将所有的参数解析和错误处理留在了 src/main.rs 中
  //测试驱动开发的模式来逐步增加minigrep的搜索逻辑
  //编写失败测试

}
