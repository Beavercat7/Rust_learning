//执行这个宏时,程序会打印一个错误信息,展开并清理栈数据
use std::fs::File;
use std::io::ErrorKind;
use std::io;
use std::fs;
use std::io::Read;
use std::error::Error;
//一个函数使用match将错误返回给代码调用者
fn read_username_from_file() -> Result<String,io::Error>{
    
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s){
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }

}
//与上述代码的等价变化
//一个使用?运算符向调用者返回错误的函数
/*
? 运算符: 如果 Result 的值是 Ok，这个表达式将会返回 Ok 中的值而程序将继续执行。如果值是 Err，Err 将作为整个函数的返回值，
*/

/*
? 运算符所使用的错误值被传递给了 from 函数，它定义于标准库的 From trait 中，其用来将错误从一种类型转换为另一种类型。当 ? 运算符调用 from 函数时，收到的错误类型被转换为由当前函数返回类型所指定的错误类型。这在当函数返回单个错误类型来代表所有可能失败的方式时很有用，即使其可能会因很多种原因失败。只要每一个错误类型都实现了 from 函数来定义如何将自身转换为返回的错误类型，? 运算符会自动处理这些转换。
*/
fn read_username_from_file_1() -> Result<String,io::Error>{
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
//链式方法调用进一步缩短代码
//?运算符之后的链式方法调用
fn read_username_from_file_2()->Result<String,io::Error>{
    let mut s =String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

fn read_username_from_file_3()->Result<String,io::Error>{
  
    fs::read_to_string("hello.txt")

}
// fn main() {
    
    
//     //panic!("crash and burn");
    
    
//     /* 
//     在这个例子中,被指明的那一行是我们代码的一部分。
//     而在其他情况下,panic!可能会出现在我们代码所调用的代码中,错误
//     信息报告的文件名和行号可能指向别人代码中的panic!宏调用。
//     */

//     //使用panic!的backtrace来寻找代码中出问题的地方。
    
    
//     // let v = vec![1,2,3];

//     // v[99];


//     //RUST_BACKTRACE=1 cargo run 
//     //通过设置RUST_BACKTRACE环境变量来得到一个backtrace
//     //backtrace是一个执行到目前位置所有被调用的函数的列表
    
//     //let f = File::open("hello.txt");
//     //小技巧:如何知道File::open返回一个Result呢?
//     //我们可以查看标准库API文档或者直接问编译器
//     /*
//     如果给 f 某个我们知道 不是 函数返回值类型的类型标注，接着尝试编译代码，编译器会告诉我们类型不匹配。然后错误信息会告诉我们 f 的类型 应该 是什么。
//     */

//     //let f:u32 = File::open("hello.txt");

//    // let f = File::open("hello.txt");

//     //使用match表达式处理可能会返回的Result成员
//     // let f = match f{
//     //     Ok(file) => file,
//     //     Err(error) => {
//     //         panic!("Problem opening the file: {:?}",error);
//     //     },
//     // };
    
//     // let f = File::open("hello.txt");
//     // let f = match f{
//     //     Ok(file) => file,
//     //     Err(error) => match error.kind(){
//     //         ErrorKind::NotFound => match File::create("hello.txt"){
//     //            Ok(fc) => fc,
//     //            Err(e) => panic!("Problem creating the file: {:?}",e), 
//     //         },
//     //         other_error => panic!("Problem opening the file: {:?}",other_error),
//     //     },
//     // };

//     /*
//     消除大量嵌套match表达式的等价代码
//     let f = File::open("hello.txt").unwrap_or_else(|error| {
//         if error.kind() == ErrorKind::NotFound {
//             File::create("hello.txt").unwrap_or_else(|error| {
//                 panic!("Problem creating the file: {:?}", error);
//             })
//         } else {
//             panic!("Problem opening the file: {:?}", error);
//         }
//     });
//     */

//     //失败时panic的简写:unwarp和expect
//     //let f = File::open("hello.txt").unwrap();
    
//     //let f = File::open("hello.txt").expect("Failed to open hello.txt");

//     //如果在多处使用 unwrap，则需要花更多的时间来分析到底是哪一个 unwrap 造成了 panic，因为所有的 unwrap 调用都打印相同的信息。

//     //传播错误
//     //比起你代码所拥有的上下文，调用者可能拥有更多信息或逻辑来决定应该如何处理错误。

//     //传播错误的简写:?运算符

//     //?运算符可被用于返回Result的函数
//     //?运算符可被用于返回值类型为Result的函数

//     let f = File::open("hello.txt")?;
// }

//当你期望在不返回 Result 的函数中调用其他返回 Result 的函数时使用 ? 的话，有两种方法修复这个问题。一种技巧是将函数返回值类型修改为 Result<T, E>，如果没有其它限制阻止你这么做的话。另一种技巧是通过合适的方法使用 match 或 Result 的方法之一来处理 Result<T, E>。
fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?;

    Ok(())
}



