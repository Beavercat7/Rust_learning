//String和字符串slice都是UTF-8编码

fn main() {
   //新建字符串
   //新建一个空的String
   let mut s = String::new();
   //使用to_string方法从字符串字面量创建String
   let data = "initial contents";
   let s = data.to_string();
    
   //该方法也可以直接用于字符串字面量
   let s = "initial contents".to_string();
   
   //使用String::from函数从字符串字面值创建String

   //更新字符串,用push_str和push附加字符串

   //可以通过push_str方法来附加字符串slice,从而使String变长
   let mut s = String::from("foo");
   s.push_str("bar");

   //将字符串slice的内容附加到String后使用它
   let mut s1 = String::from("foo");
   let s2 = "bar";
   s1.push_str(s2);
   //push_str的参数是引用
   println!("s2 is {}",s2);

   //使用push将一个字符加入String值中
   let mut s = String::from("lo");
   s.push('l');

   //使用+运算符或format!宏拼接字符串
   let s1 = String::from("Hello, ");
   let s2 = String::from("world!");
   let s3 = s1 + &s2;//注意s1被移动了,不能继续使用
   /*
   s1在相加以后不再有效的原因,和使用s2的引用的原因，与使用+运算符时调用的函数
   签名有关。+运算符使用了add函数。
   fn add(self,s:&str)->String{

   }
   &s2类型是&String
   能在add调用使用&s2,是因为&String可以被强转成&str
   当add函数被调用时,Rust使用了一个被称为解引用强制转换的技术
   &s2 => &s2[..]
   因为add没有获取参数的所有权,所以s2在这个操作后仍然是有效的String

   可以发现add获取了self的所有权,因为self没有使用&。
   这意味着s1的所有权将被移动到add调用中,之后不再有效。
   let s3 = s1 + &s2
   这个语句会获取s1的所有权,附加上从s2中拷贝的内容,并返回结果的所有权。

   */

   let s1 = String::from("tic");
   let s2 = String::from("tac");
   let s3 = String::from("toe");
   let s = s1 + "_" + &s2 + "_" + &s3;
   //获取了s1的所有权
   let s = format!("{}-{}-{}",s1,s2,s3);
   //返回一个带结果内容的String，并且不会获取任何参数的所有权

   //索引字符串
//    let  s1 = String::from("hello");

//    let h = s1[0];
//  Rust的字符串不支持索引

//  内部表现
//  String是一个Vec<u8>的封装
    let len = String::from("Hola").len();
    // 字节 标量值  字形簇
    // 每个Unicode标量值可能需要多个字节存储
    // 最接近人们眼中字母的概念是字形簇
    // 字形簇可能由多个标量值组成
   /*
   最后一个 Rust 不允许使用索引
   获取 String 字符的原因是，
   索引操作预期总是需要常数时间 (O(1))。
   但是对于 String 不可能保证这样的性能，
   因为 Rust 必须从开头到索引位置遍历
   来确定有多少有效的字符。

   */
    // let hello = "Здравствуйте";
    // let s = &hello[0..4];

    // 遍历字符串的办法
    // 操作单独的Unicode标量值可以使用chars方法
    for c in "नमस्ते".chars() {
    println!("{}", c);
    }
    // bytes办法返回每一个原始字节
    for b in "नमस्ते".bytes() {
    println!("{}", b);
    }
}

