//在函数定义中使用泛型
//引入泛型参数消除重复
fn largest<T>(list:&[T])->T{
    let mut largest = list[0];

    for &item in list.iter()
    {
        if item > largest{
            largest = item;
        }
    }
    largest
}
// 结构体定义中的泛型
struct Point<T>{
    x:T,
    y:T,
}
impl<T>Point<T>{
    fn x(&self) -> &T{
        &self.x
    }
    
}
// 可以选择为Point<f32>实例实现方法,而不是为泛型Point实例
//构建一个只用于拥有泛型参数T的结构体的具体类型的impl块
impl Point<f32>{
    fn distance_from_origin(&self) -> f32{
        (self.x.powi(2)+self.y.powi(2)).sqrt()
    }
}
// 定义有两个泛型类型参数T和U的结构体
struct Point1<T,U>{
    x:T,
    y:U,
}
//结构体定义中的泛型类型参数并不总是与结构体方法签名中使用的泛型是同一类型
//泛型通过 impl 声明而另一些通过方法定义声明的情况。这里泛型参数 T 和 U 声明于 impl 之后，因为他们与结构体定义相对应。而泛型参数 V 和 W 声明于 fn mixup 之后，因为他们只是相对于方法本身的。
impl<T,U>Point<T,U>{
    fn mixup<V,W>(self,other:Point<V,W>)->Point<T,W>{
        Point{
            x:self.x,
            y:other.y,
        }
    }
}
fn main() {
    let number_list = vec![34,50,25,100,65];

    let result = largest(&number_list);
    
    println!("The largest number is {}",result);

    let char_list = vec!['y','m','a','q'];
    
    let result = largest(&char_list);

    println!("The largest char is {}",result);

    let integer = Point{x:5,y:10};
    let float = Point{x:1.0,y:4.0};

    //使用两个泛型的Point,这样x和y可能是不同类型
    let both_integer = Point1{x:5,y:10};
    let both_float = Point1{x:1.0,y:4.0};
    let integer_and_float = Point1{x:5,y:4.0};

    let p = Point{x:5,y:10};
    println!("p.x = {}",p.x());

    let p1 = Point1{x:5,y:10.4};
    let p2 = Point1{x:"Hello",y:'c'};
    let p3 = p1.mixup(p2);

    println!("p3.x = {},p3.y = {}",p3.x,p3.y);

}
/*
泛型代码的性能:
Rust通过在编译时进行泛型代码的单态化来保证效率
单态化是一个通过填充编译时使用的具体类型,将通用代码转换成特定代码的过程。
*/
//我们可以使用泛型来编写不重复的代码，而 Rust 将会为每一个实例编译其特定类型的代码。这意味着在使用泛型时没有运行时开销；当代码运行，它的执行效率就跟好像手写每个具体定义的重复代码一样。这个单态化过程正是 Rust 泛型在运行时极其高效的原因。
