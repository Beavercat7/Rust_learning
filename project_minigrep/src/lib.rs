use std::env;
use std::fs;
use std::error::Error;
//重构改进模块化和错误处理
//提取参数解析器
//从main中提取出parse_config函数
pub struct Config{
   pub query:String,
   pub filename:String,
}
//从new中返回Result而不是调用panic!
//&'static str是字符串字面值的类型,也是目前的错误信息
//从main提取逻辑
impl Config{
    pub fn new(args:&[String])->Result<Config,&'static str>{
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
pub fn run(config:Config)->Result<(),Box<dyn Error>>{
   let contents = fs::read_to_string(config.filename)?;
   for line in search(&config.query,&contents){
      println!("{}",line);;
   }
   Ok(())
}
//开始编写能使测试通过的代码
pub fn search<'a>(query:&str,contents:&'a str)->Vec<&'a str>{
   let mut results = Vec::new();
   //使用line方法遍历每一行
   for line in contents.lines(){
      //用查询字符串搜索每一行(增加检查文本行是否包含query中字符串的功能)
      if line.contains(query){
         //存储匹配的行
         results.push(line);
      }
   }
   results
}
#[cfg(test)]
mod tests{
   use super::*;

   #[test]
   fn one_result(){
      let query = "duct";
      let contents = "\
Rust: 
safe,fast,productive.
Pick three.";
      assert_eq!(vec!["safe,fast,productive."],search(query,contents));
   }
   //在run函数中使用search函数
}