//trait告诉Rust编译器某个特定类型拥有可能与其他类型共享的功能
//可以通过trait以一种抽象的方式定义共享的行为。

//Summary trait定义,它包含由summarize方法提供的行为。
//rait 体中可以有多个方法：一行一个方法签名且都以分号结尾。
pub struct NewArticle{
    pub headline:String,
    pub location:String,
    pub author:String,
    pub content:String,
}

//在NewArticle和Tweet类型上实现Summary trait
impl Summmary for NewArticle{
    fn summarize(&self) -> String{
        format!("{}, by {} ({})",self.headline,self.author,self.location);
    }
}

pub struct Tweet{
    pub username:String,
    pub content:String,
    pub reply:bool,
    pub retweet:bool,
}

// impl Summmary for Tweet{
//     fn summarize(&self)->String{
//         format!("{}: {}",self.username,self.content);
//     }
// }
//Summary trait的定义,带有一个summarize方法的默认实现
// pub trait Summary{
//     fn summarize(&self) -> String{
//         String::from("(Read more...)")
//     }
// }
// impl Summmary for NewArticle{

// }

pub trait Summary{
    fn summarize_author(&self)->String;

    //默认实现
    fn summarize(&self)->String{
        format!("(Read more from {}...)",self.summarize_author())
    }
    //为了使用这个版本的Summary,只需在实现trait时定义summarize_author即可
  
}
//trait作为参数
//如何使用trait来接受多种不同类型的参数
pub fn notify(item:impl Summary){
    println!("Breaking news! {}",item.summarize());
}

//Trait Bound语法
//impl Trait语法适用于直观的例子,它实际上是一种较长形式的语法糖,我们称为bound
// pub fn notify<T:Summary>(item:T){
    
// }
// pub fn notify(item1: impl Summary,item2:impl Summary){}
// 这适用于item1和item2允许是不同类型的情况（只要它们都实现了Summary）
// 不过如果你希望强制它们是相同类型呢
// pub fn notify<T:Summary>(item1:T,item:T){}
// 泛型T被指定为item1和item2的参数限制

// 通过+指定多个trait bound
// 如果notify需要显示item的格式化形式,同时也要使用summarize方法
// 那么item就需要同时实现两个不同的trait: Display 和 Summary。 这可以通过+语法实现:
// pub fn notify(item:impl Summary + Dispaly){}
// +语法也适用于泛型的trait bound
// pub fn notify<T:Summary + Display>(item:T){}

// 通过where简化trait bound
// 使用过多的trait bound也有缺点
// 在函数签名之后where从句中指定trait bound的语法
// fn some_function<T:Display + Clone,U:Clone+Debug>(t:T,u:U)->i32
/*
   fn some_function<T,U>(t:T,u:U)->i32
   where
   {
     T:Display + Clone,
     U:Clone + Debug
   }
 
*/
use std::fmt::Dispaly;
struct Pair<T>{
    x:T,
    y:T,
}
impl<T>Pair<T>{
    fn new(x:T,y:T)->self{
        x,
        y,
    }
}
impl<T:Dispaly + PartialOrd>Pair<T>{
    fn cmp_dispaly(&self){
        if self.x >= self.y{
            println!("")
        }
        else {
            
        }
    }
}


impl Summmary for Tweet{
    fn summarize_author(&self)->String{
        format!("@{}",self.username)
    }
}
//请注意，无法从相同方法的重载实现中调用默认方法。

//实现 trait 时需要注意的一个限制是，只有当 trait 或者要实现 trait 的类型位于 crate 的本地作用域时，才能为该类型实现 trait。
//use crate::Summmary
