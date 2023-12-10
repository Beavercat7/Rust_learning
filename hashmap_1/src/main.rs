use std::collections::HashMap;

fn main() {
    // 新建一个哈希map并插入一些键值对
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"),10);
    scores.insert(String::from("Yellow"),50);
    
    let teams = vec![String::from("Blue"),String::from("Yellow")];
    let initial_scores = vec![10,50];

    let scores:HashMap<_,_> = teams.iter().zip(initial_scores.iter()).collect();
    //元组的vecotr的collect办法
    //其中每个元组包含一个键值对
    //zip创建一个元组的vector

    //哈希map和所有权
    /*
    对于i32这样实现了copy trait的类型,其值可以拷贝进哈希map
    对于像String这样拥有所有权的值,其值将被移动而哈希map会成
    为这些值的所有者。
    */

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name,field_value);
    //这里field_name和field_value不再有效
    //一旦键值对被插入以后就成为哈希map所拥有


    //访问哈希map中的值
    //可以通过get方法并提供对应的键来从哈希map中获取值
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"),10);
    scores.insert(String::from("Yellow"),50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    //访问哈希map中储存的蓝队分数
    //get返回Option<V>

    //任意顺序打印每个键值对
    for (key,value) in &scores{
        println!("{}: {}",key,value);
    }

    //更新哈希map
    // 覆盖一个值
    scores.insert(String::from("Blue"),10);
    scores.insert(String::from("Blue"),25);

    println!("{:?}",scores);


    //只在键没有对应值时插入
    //Entry的or_insert办法在键对应的值存在时就返回这个值的可变引用,如果不存在则将参数
    //作为新值插入并返回新值的可变引用。
    let mut scores = HashMap::new();
    
    scores.insert(String::from("Blue"),50);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}",scores);

    //根据旧值更新一个值

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace(){
        let count = map.entry(word).or_insert(0);
        *count+=1;
    }
    println!("{:?}",map);
}   
