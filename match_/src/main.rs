
//一个枚举和一个以枚举成员作为模式的match表达式
// fn value_in_cents(coin:Coin)->u8{
//     match coin{
//         Coin::Penny=>1,
//         Coin::Nickel=>5,
//         Coin::Dime=>10,
//         Coin::Quarter=>25,
//     }
// }

//一个在Option<i32>上使用match表达式的函数
fn plus_one(x:Option<i32>)->Option<i32>{
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}


fn main() {
    let five = Some(5);
    println!("five = {:?}",five);
    let six = plus_one(five);
    println!("six = {:?}",six);
    let none = plus_one(None);

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        num => move_player(num),
    }
    
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }

    match dice_roll{
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ =>reroll(),
    }
}
fn add_fancy_hat(){}
fn remove_fancy_hat(){}
fn reroll(){}
fn move_player(num_spaces:u8){
    println!("num_spaces = {}",num_spaces);
}
//绑定值的模式
//匹配分支的另一个有用的功能是可以绑定匹配的模式的部分值
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

fn value_in_cents1(coin:Coin) -> u8{
    match coin{
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) =>{
            println!("State quatter from {:?}!",state);
            25
        }
    }
}