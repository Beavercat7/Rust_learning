//User结构体定义
struct User
{
    active:bool,
    usename:String,
    email:String,
    sign_in_count:u64,
}
//build_user函数获取email和用户名并返回User实例
fn build_user(email:String,username:String)->User{
    User{
        email:email,
        username:username,
        active:true,
        sign_in_count:1,
    }
}
//变量和字段同名时的字段初始化简写语法
fn build_user(email:String,username:String)->User{
    User{
        email,
        username,
        active:true,
        sign_in_count:1,
    }
}
fn main() {
//创建User结构体的实例
    let user1 = User{
        email:String::from("someone@example.com");
        username:String::from("someusername123");
        active:true,
        sign_in_count:1,
    };
//改变User实例email字段的值
//整个实例必须是可变的，Rust并不允许只将某个字段标记为可变
    let mut user1 = User{
        email:String::from("someone@example.com"),
        username:String::from("someusename123"),
        active:true,
        sign_in_count:1,
    };
    user1.email = String::from("anotheremail@example.com");
//使用结构体更新语法从其他实例创建实例
//使用user1中的一个值创建一共新的User实例
    let user2 = User{
        active:user1.active,
        usename:user1.usename,
        email:String::from("another@example.com"),
        sign_in_count:user1.sign_in_count,
    };
    //使用结构体更新语法为一个User实例设置一个新的email值
    //不过其余值来自user1变量中实例的字段
    let user2 = User{
        email:String::from("another@example.com"),
        ..user1
    };
    //
}





