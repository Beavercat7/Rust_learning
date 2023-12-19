//Rc<T>引用计数智能指针
//单个值可能会有多个所有者
//为了启用多所有权,Rust有一个Rc<T>的类型
//称其为引用计数。

//使用Rc<T>共享数据
//Rc<T>只能用于单线程场景
use crate::List::{Cons,Nil};
use std::rc::Rc;
enum List{
    Cons(i32,Rc<List>),
    Nil,
}
fn main() {
    //克隆Rc<T>会增加引用计数
    let a = Rc::new(Cons(5,Rc::new(Cons(10,Rc::new(Nil)))));
    println!("count after creating a = {}",Rc::strong_count(&a));
    let b = Cons(3,Rc::clone(&a));
    println!("count after creating b = {}",Rc::strong_count(&a));
    {
        let c = Cons(4,Rc::clone(&a));
        println!("count after creating c = {}",Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}",Rc::strong_count(&a));

}
