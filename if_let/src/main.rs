#[derive(Debug)]
enum UsState{
    Alabama,
    Alaska,
}

//Quarter成员也存放了一个UsState值的Coin枚举
enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

//处理只匹配一个模式的值而忽略其他模式的值。
fn main() {
   let some_u8_value = Some(0u8);
   match some_u8_value{
    Some(3) => println!("three"),
    _ => (),
   }
   //等价行为
   if let Some(3) = some_u8_value{
     println!("three");
   }

//    let mut count = 0;
//    match coin{
//     Coin::Quarter(state) => println!("State quarter from {:?}",state);
//     _ => count+=1;
//    }

//    let mut count = 0;
//    if let Coin::Quarter(state) = coin{
//       println!("State quarter from {:?}",state);
//    }
//    else {
//     count += 1;
//    }
}
