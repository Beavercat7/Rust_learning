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

impl Summmary for Tweet{
    fn summarize_author(&self)->String{
        format!("@{}",self.username)
    }
}
//请注意，无法从相同方法的重载实现中调用默认方法。

//实现 trait 时需要注意的一个限制是，只有当 trait 或者要实现 trait 的类型位于 crate 的本地作用域时，才能为该类型实现 trait。
//use crate::Summmary
