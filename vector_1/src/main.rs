fn main() {
    ///新建vector
    //新建一个空的vector来储存i32类型的值
    let v:Vec<i32> = Vec::new();
    //新建一个包含初值的vecotr
    let v = vec![1,2,3];

    ///更新vector
    //对于新建一个vector并向其添加元素,可以使用push
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    
    ///丢弃vector时也会丢弃其所有元素
    {
        let v = vec![1,2,3,4];

        //处理变量v
    }// <-这里v离开作用域并被丢弃

    let v = vec![1,2,3,4,5];

    let third:&i32 = &v[2];
    println!("The third element is {}",third);

    match v.get(2)
    {
        Some(third) =>println!("The third element is {}",third),
        None => println!("There is no third element"),
    }

    //尝试访问一个包含5个元素的vector的索引100处的元素
     let v = vec![1,2,3,4,5];

     let does_not_exist = &v[100];
     let does_not_exist = v.get(100);


    //当我们获取了vector的第一个元素的不可变引用并尝试在vector末尾增加一个元素的时候,这是行不通的
    //在拥有vector中项的引用的同时向其增加一个元素
    // let mut v = vec![1,2,3,4,5];

    // let first = &v[0];

    // v.push(6);

    // println!("The first element is: {}",first);
    /* 
    vector的工作方式:
    增加新元素的时候,在没有足够空间将所有元素依次相邻存放的情况下，
    可能会要求分配新内存并将老的元素拷贝到新的空间中。
    这时，第一个元素的引用就指向了被释放的内存，借用规则阻止程序陷入这种状况。
    */
    
    
    ///遍历vector的元素
    //通过for循环遍历vector的元素并打印
    let v = vec![100,32,57];
    for i in &v{
        println!("{}",i);
    }

    //遍历可变vector的每一个元素的可变引用以遍能改变它们
    let mut v = vec![100,32,57];
    for i in &mut v{
        *i += 50;
    }

    let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
    ];
}

///使用枚举来储存多种类型
//枚举的成员都被定义为相同的枚举类型
enum SpreadsheetCell{
    Int(i32),
    Float(f64),
    Text(String),
}




