enum IpAddrKind{
    V4,
    V6,//枚举的成员
}
//将IP地址的数据和Ipaddrkind成员存储在一个struct中
struct Ipaddr{
    kind:IpAddrKind,
    address:String,
}

//使用枚举并将数据直接放进每一个枚举成员
//而不是将枚举作为结构体的一部分
enum Ipaddr{
    V4(String),
    V6(String),
}

//用枚举替代结构体
//每个成员可以处理不同类型和数量的数据。
enum Ipaddr1{
    V4(u8,u8,u8,u8),
    V6(String),
}

//一个Message枚举,其每个成员都存储了不同数量和类型的值
enum Message{
    Quit,
    Move{x:i32,y:i32},
    Write(String),
    ChangeColor(i32,i32,i32),
}

impl Message
{
    fn call(&self)
    {
        //在这里定义方法体
    }
}
fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    /*
    注意枚举的成员位于其标识符的命名空间中,并使用两个冒号分开。
    */
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    let home = Ipaddr{
        kind:IpAddrKind::V4,
        address:String::from("127.0.0.1"),
    };
    let loopback = Ipaddr{
        kind:IpAddrKind::V6,
        address:String::from("::1"),
    }
    let home = Ipaddr::V4(String::from("127.0.0.1"));

    let loopback = Ipaddr::V6(String::from("::1"));
 
    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(5);
    let some_string = Somme("a string");

    let absent_number:Option<i32> = None;
}


fn route(ip_type:IpAddrKind){}

//option枚举
//Rust并没有很大其他语言中有的空值功能,空值(NULL)是一个值,它代表没有值。
/*
enum Option<T>
{
    Some(T),
    None,
}

*/