mod front_of_house{
   pub mod hosting{
    pub fn add_to_waitlist(){}
   }
}
pub fn eat_at_restaurant(){
    //绝对路径
    crate::front_of_house::hosting::add_to_waitlist();

    //相对路径
    front_of_house::hosting::add_to_waitlist();
}

//使用super起始的相对路径
// fn server_order() {}

// mod back_of_house {
//     fn fix_incorrect_order(){
//         cook_order();
//         super::server_order();
//     }
//     fn cook_order(){}
// }

//创建公有的结构体和枚举
// mod back_of_house{
//     pub struct Breakfast{
//         pub toast:String,
//         seasonal_fruit:String,
//     }

//     impl Breakfast{
//         pub fn summer(toast:&str)-> Breakfast{
//             Breakfast{
//                 toast:String::from(toast);
//                 seasonal_fruit:String::from("peaches"),
//             }
//         }
//     }
// }

// pub fn eat_at_restaurant(){
//     //在夏天点一份黑麦面包作为早餐
//     let mut meal = back_of_house::Breakfast::summer("Bye");
//     //更改我们想要的面包
//     meal.toast = String::from("Wheat");
//     println!("I'd like {} toast pleas",meal.toast);

//     // 如果取消下一行的注释，将会导致编译失败；我们不被允许
//     // 看到或更改随餐搭配的季节水果
//     // meal.seasonal_fruit = String::from("blueberries");
// }

//如果我们将枚举设为公有,则它的所有成员都将变为公有
// mod back_of_house {
//     pub enum Appetizer {
//         Soup,
//         Salad,
//     }
// }

// pub fn eat_at_restaurant() {
//     let order1 = back_of_house::Appetizer::Soup;
//     let order2 = back_of_house::Appetizer::Salad;
// }
//使用use将模块引入作用域
mod front_of_house{
    pub mod hosting{
        pub fn add_to_waitlist(){}
    }
}
use crate::front_of_house::hosting;

pub fn eat_at_restaurant(){
    hosting::add_to_waitlist();
}

/*
引入枚举、结构体等项时
将HashMap引入作用域的习惯用法
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}
*/

/*

use std::fmt;
use std::io;
使用父模块将两个具有别名的类型引入同一作用域
fn function1() -> fmt::Result {
    // --snip--
}

fn function2() -> io::Result<()> {
    // --snip--
}
*/

/*
使用as关键字提供新的名称

use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
}

fn function2() -> IoResult<()> {
    // --snip--
}

*/

/*
使用pub use重导出名称
当使用 use 关键字将名称导入作用域时，在新作用域中可用的名称是私有的。如果为了让调用你编写的代码的代码能够像在自己的作用域内引用这些类型，可以结合 pub 和 use。这个技术被称为 “重导出（re-exporting）”，因为这样做将项引入作用域并同时使其可供其他代码引入自己的作用域。
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
通过pub use使名称可引入任何代码的作用域中
*/

/*
嵌套路径来消除大量的use行
use std::cmp::Ordering;
use std::io;
use std::{cmp::Ordering, io};

use std::io;
use std::io::Write;
use std::io::{self, Write};

*/

/*
通过glob运算符将所有的公有定义引入作用域
如果希望将一个路径下所有公有项引入作用域,
可以指定路径后跟glob运算符*
use std::collection::*;
*/
