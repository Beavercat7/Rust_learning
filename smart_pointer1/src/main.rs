//使用Box<T>在堆上储存数据
//Box<T>实现了Deref和Drop trait 可以被当作引用以及离开作用域自己释放(栈+堆)内存
//处理递归类型的数据
enum List{
    Cons(i32,Box<List>),
    Nil,
}
use crate::List::{Cons,Nil};
use std::ops::Deref;
//因为Cons存放一个Box所以List不是无限大小了
struct MyBox<T>(T);
impl<T>MyBox<T>{
    fn new(x:T)->MyBox<T>{
        MyBox(x)
    }
}
//通过实现Deref trait将某类型像引用一样处理
impl<T> Deref for MyBox<T>{
    type Target = T;

    fn deref(&self) -> &T{
        &self.0
    }
} 
fn hello(name:&str){
    println!("Hello, {}",name);
}
struct CustomSmartPointer{
    data:String,
}
//结构体CustomSmartPointer，实现了放置清理代码的Drop trait
impl Drop for CustomSmartPointer{
    fn drop(&mut self)
    {
        println!("Dropping CustomSmartPointer with data '{}'!",self.data);
    }
}
fn main() {
   let list = Cons(1,Box::new(Cons(2,Box::new(Cons(3,Box::new(Nil))))));
   //通过Deref trait将智能指针当作常规引用处理
   //像引用一样使用Box<T>
   let x = 5;
   let y = Box::new(x);
   assert_eq!(5,x);
   assert_eq!(5,*y);
   //尝试使用引用和Box<T>相同的方式使用MyBox<T>
   let x = 5;
   let y = MyBox::new(x);
   assert_eq!(5,x);
   assert_eq!(5,*y);
//函数和方法的隐式解引用强制转换
//强制转换的类型是 type Target = () ;关联类型
//当所涉及到的类型定义了 Deref trait，Rust 会分析这些类型并使用任意多次 Deref::deref 调用以获得匹配参数的类型。这些解析都发生在编译时，所以利用解引用强制转换并没有运行时损耗！
//因为解引用强制转换,使用MyBox<String>的引用调用hello是可行的  
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    //hello(&(*m)[..]);不使用隐式解引用强制转换

    //类似于使用Deref trait重载不可变引用的*运算符
    //DerefMut trait用于重载可变引用的*运算符
    /*
    当 T: Deref<Target=U> ：从 &T 到 &U。
    当 T: DerefMut<Target=U> ：从 &mut T 到 &mut U。
    当 T: Deref<Target=U> ：从 &mut T 到 &U。
    */
    //使用Drop Trait运行清理代码
    //结构体CustomSmartPointer,其实现了放置清理代码的Drop trait
    let c = CustomSmartPointer{data:String::from("my stuff")};
    let d = CustomSmartPointer{data:String::from("other stuff")};
    println!("CustomSmartPointer created.");

    //尝试手动调用Drop trait的drop方法提早清理
    drop(c);

}
