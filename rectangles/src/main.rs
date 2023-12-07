///一个使用结构体的示例程序
#[derive(Debug)]
struct Rectangle{
   width:u32,
   height:u32,
}
fn main()
{
    let width1 = 30;
    let height1 = 50;

    println!("The area of the rectangle is {} square pixeles.",area(width1,height1));
    
    //使用元组重构
    let rect1 = (30,50);

    println!("The area of the rectangle is {} square pixels.",area1(rect1));

    //使用结构体重构:赋予更多意义
    
    let rect2 = Rectangle{
        width:30,
        height:50,
    };
    println!("rect2 is {:#?}",rect2);

    //使用Debug格式打印数值的方法是使用dbg!宏
    //dbg!宏接收一个表达式的所有权，打印出代码中调用dbg!宏时所在的文件和行号
    //以及该表达式的结果值,并返回该值的所有权。
    let scale = 2;
    let rect3 = Rectangle{
        width:dbg!(30*scale),
        height:50,
    };

    dbg!(&rect3);
}

fn area(width:u32,height:u32)->u32{
    width*height
}

fn area1(dimensions:(u32,u32))->u32{
    dimensions.0*dimensions.1
}




