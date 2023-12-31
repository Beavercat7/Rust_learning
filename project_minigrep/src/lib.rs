use std::env;
use std::fs;
use std::error::Error;
//重构改进模块化和错误处理
//提取参数解析器
//从main中提取出parse_config函数
pub struct Config{
   pub query:String,
   pub filename:String,
   pub case_sensitive:bool,
}
//从new中返回Result而不是调用panic!
//&'static str是字符串字面值的类型,也是目前的错误信息
//从main提取逻辑

//args:&[String]
impl Config{
    pub fn new(mut args:std::env::Args)->Result<Config,&'static str>{
      args.next();

      let query = match args.next(){
         Some(arg) => arg,
         None => return Err("Didn't get aa query string"), 
      };

      let filename = match args.next(){
         Some(arg) => arg,
         None => return Err("Didn't get a file name"),
      };

      // if args.len() < 3{
      //    //panic!("not enough arguments");
      //    return Err("not enough arguments");
      // }
      //let query = args[1].clone();
      //let filename = args[2].clone();
      //检查叫做CASE_INSENSITIVE的环境变量
      let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
     
      Ok(Config{query,filename,case_sensitive})
   }
}
//现在run函数包含了main中从读取文件开始的剩余的所有逻辑
//run函数获取一个Config实例作为参数。
//从run函数中返回错误
//修改run函数返回Result
//根据config.case_sensitive的值调用search或search_case_insenstive
//最后需要实际检测环境变量。
pub fn run(config:Config)->Result<(),Box<dyn Error>>{
   let contents = fs::read_to_string(config.filename)?;
   let results = if config.case_sensitive{
      search(&config.query,&contents)
   }
   else 
   {
      search_case_insensitive(&config.query,&contents)
   };
   for line in results{
      println!("{}",line);
   }
   Ok(())
}
//最后需要实际检查环境变量


//开始编写能使测试通过的代码
//在search函数实现中使用迭代器适配器
pub fn search<'a>(query:&str,contents:&'a str)->Vec<&'a str>{
   // let mut results = Vec::new();
   // //使用line方法遍历每一行
   // for line in contents.lines(){
   //    //用查询字符串搜索每一行(增加检查文本行是否包含query中字符串的功能)
   //    if line.contains(query){
   //       //存储匹配的行
   //       results.push(line);
   //    }
   // }
   // results
   //通过使用迭代器适配器方法来编写更简明的代码,避免可变的中间result vector的使用
   //函数式编程风格倾向于最小化可变状态的数量来使得代码更简洁。
   //去掉可变状态可能会使得进行并行搜索的增强会更容易,因为我们不必管理results vector的并发访问
   contents.lines()
   .filter(|line|line.contains(query))
   .collect()
}
//它在比较查询和每一行之前将他们都转换为小写
pub fn search_case_insensitive<'a>(query:&str,contents:&'a str)->Vec<&'a str>{
   let query = query.to_lowercase();
   // let mut results = Vec::new();

   // for line in contents.lines()
   // {
   //    if line.to_lowercase().contains(&query){
   //       results.push(line);
   //    }
   // }
   // results
   //自己根据上诉函数完成的迭代器适配器改造!
   contents.lines().filter(|line|line.to_lowercase().contains(&query.to_lowercase())).collect()
}
//处理环境变量
//我们将增加一个额外的功能来改进 minigrep：用户可以通过设置环境变量来设置搜索是否是大小写敏感的 。
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

   #[test]
   fn case_sensitive(){
      let query = "duct";
      let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
      assert_eq!(vec!["safe, fast, productive."],search(query,contents));
   }

   #[test]
   fn case_insensitive(){
      let query = "rUsT";
      let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
      assert_eq!(vec!["Rust:","Trust me."],search_case_insensitive(query,contents));
   }
}
//为准备添加的大小写不敏感函数新增失败测试

//检查错误应该写入何处
//将错误打印到标准错误

