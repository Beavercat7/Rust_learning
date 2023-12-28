 //Iterator trait和next方法
//  pub trait Iterator{
//         type Item;
        
//         fn next(&mut self)-> Option<Self::Item>;
// }
#[derive(PartialEq,Debug)]
struct Shoe{
    size:u32,
    style:String,
}
fn shoes_in_my_size(shoes:Vec<Shoe>,shoe_size:u32)->Vec<Shoe>{
    shoes.into_iter()
    .filter(|s|s.size == shoe_size)
    .collect()
}
#[test]
fn filter_by_size(){
    let shoes = vec![
    Shoe{size:10,style:String::from("sneaker")},
    Shoe{size:13,style:String::from("sandal")},
    Shoe{size:10,style:String::from("boot")},    
    ];

    let in_my_size = shoes_in_my_size(shoes,10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe{size:10,style:String::from("sneaker")},
            Shoe{size:10,style:String::from("boot")},
        ]
    )
}
//定义Counter结构体和一个创建count初值为0的Counter实例的new函数
struct Counter{
    count:u32,
}
impl Counter{
    fn new() -> Counter{
        Counter{count:0}
    }
}

//接下来为Counter类型实现Iterator trait,通过定义next方法来指定迭代器时的行为
impl Iterator for Counter{
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item>{
    self.count += 1;
    if self.count < 6{
        Some(self.count)
    }
    else{
        None
    }
    }
}
//演示使用Counter结构体的迭代器功能
//测试next方法实现的功能
#[test]
fn calling_next_directly(){
    let mut counter = Counter::new();

    assert_eq!(counter.next(),Some(1));
    assert_eq!(counter.next(),Some(2));
    assert_eq!(counter.next(),Some(3));
    assert_eq!(counter.next(),Some(4));
    assert_eq!(counter.next(),Some(5));
    assert_eq!(counter.next(),None);
}
//使用自定义迭代器中其他Iterator trait方法
//通过定义next方法实现Iterator trait,我们现在就可以使用任何标准库定义的拥有默认实现的Iterator trait方法,因为他们都使用了next方法的功能

#[test]
fn using_other_iterator_trait_methods(){
    let sum:u32= Counter::new().zip(Counter::new().skip(1)).map(|(a,b)|a*b).filter(|x|x%3==0).sum();
    assert_eq!(18,sum);
}
fn main(){
    //创建一个迭代器
    let v1 = vec![1,2,3];
    let v1_iter = v1.iter();

    //在一个for循环中使用迭代器
    // for val in v1_iter{
    //     println!("Got: {}",val);
    // }

    //在迭代器上(直接)调用next方法
//    assert_eq!(v1_iter.next(),Some(&1));
//    assert_eq!(v1_iter.next(),Some(&2));
//    assert_eq!(v1_iter.next(),Some(&3));
//    assert_eq!(v1_iter.next(),Some(&4));
    //消费迭代器的方法

   let total:i32 = v1_iter.sum();
   
   assert_eq!(total,6);
   //调用sum方法获取迭代器所有项的总和
   //调用sum之后不再允许使用v1_iter因为调用sum时它会获取迭代器的所有权

   //产生其他迭代器的方法
   //另一类方法->迭代器适配器
   //允许我们将当前迭代器变为不同类型的迭代器
   let v2:Vec<_> = v1.iter().map(|x|x+1).collect();
   
   assert_eq!(v2,vec![2,3,4]);
}