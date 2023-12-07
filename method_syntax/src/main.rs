#[derive(Debug)]

//方法是一种关联函数,让你指定结构体的[实例]所具有的行为!!!
struct Rectangle{
    width:u32,
    height:u32,
}
impl Rectangle{
    fn area(&self)->u32{
        self.width*self.height
    }
    fn width(&self) -> bool{
        self.width > 0
    }
    fn can_hold(&self,other:&Rectangle)->bool{
        self.width > other.width && self.height > other.height
    }
    //所有在impl块中定义的函数被称为关联函数,因为它们与impl后面命名的类型相关
    //定义不以self为第一参数的关联函数(因此不是方法),因为它们并不作用于一个结构体的实例
    fn square(size:u32) -> Rectangle{
        Rectangle{
            width:size,
            height:size,
        }
    }
}
fn main() {
   let rect1 = Rectangle{
    width:30,
    height:50,
   };

   println!("The area of the rectangle is {} square pixels",rect1.area());
   
   if rect1.width()
   {
    println!("The rectangle has a nonzero width: it is {}",rect1.width);
   }

   let rect2 = Rectangle {
    width: 10,
    height: 40,
    };
    let rect3 = Rectangle {
    width: 60,
    height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    
    //使用结构体名和::语法来调用这个关联函数
    //这个方法位于结构体的命名空间中,::用于关联函数和模块创建的命名空间。
    let sq = Rectangle::square(3);
}

//****Rust的自动引用和解引用****
/*
C++ object->something() (*object).something()

当使用object.something()调用方法时,Rust会自动为object添加&、&mut、或者*
以使得object与方法签名匹配。
也就是说,这些代码等价:
p1.distance(&p2);
(&p1).distance(&p2);

这种自动引用和之所以有效,是因为方法有明确的接收者--self的类型
在给出接收者和方法名的前提下,Rust可以明确地计算出方法是仅仅读取
(&self),做出修改(&mut self),获取所有权(self)
*/


/*
多个impl块:
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
*/