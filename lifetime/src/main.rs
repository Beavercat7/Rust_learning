//生命周期标注语法
fn longest<'a>(x:&'a str,y:&'a str)-> &'a str{
    if x.len() > y .len(){
        x
    }
    else {
        y
    }
}
struct ImportantExcept<'a>{
    part:&'a str,
} 
//方法定义中的生命周期标注
//当为带有生命周期的结构体实现方法时
fn main(){
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(),string2.as_str());
      
    }  
    println!("The longest string is {}",result);

    let novel = String::from("call me Ishmael, Some years age...");
    let first_sentence =  novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcept{part:first_sentence};
    //静态生命周期
    //'static,其生命周期能够存活于整个程序期间
    //所有的字符粗字面量都拥有'staitc生命周期
    
}