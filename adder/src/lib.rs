
#[derive(Debug)]
struct Retangle{
    width:u32,
    height:u32,
}
//第五章中Retangle结构体和can_hold方法
impl Retangle{
    fn can_hold(&self,other:&Retangle)->bool{
        self.width < other.width && self.height > other.height
    }
}
pub fn greeting(name:&str)->String{
    format!("Hello !")
}
pub fn add(left: usize, right: usize) -> usize {
    left + right
}
pub fn add_two(a:i32)->i32{
    a+2
}
pub struct Guess{
    value:i32,
}
impl Guess{
    pub fn new(value:i32)->Guess{
        if value < 1 {
            panic!("Guess value must be less than or equal to 100,got {}",value);
           
        }
        else if value > 100{
            panic!("Guess value must be greater than or equal to 1,got {}.",value);
        }
        Guess{
            value
        }
    }
}   
fn prints_and_returns_10(a:i32)->i32{
        println!("I got the value {}",a);
        10
}
#[cfg(test)]
mod tests {
    use super::*;
    // #[test]
    // fn  exploration() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }
    // #[test]
    // fn another(){
    //     panic!("Make this test fail");
    // }
    // //一个can_hold的测试,检测一个较大的矩形确实能放得下一个较小的矩形
    // #[test]
    // fn larger_can_hold_smaller(){
    //     let larger = Retangle{width:8,height:7};
    //     let smaller = Retangle{width:5, height:1};

    //     assert!(larger.can_hold(&smaller));
    // }
    // #[test]
    // fn smaller_cannot_hold_large(){
    //     let larger = Retangle{width:8,height:7};
    //     let smaller = Retangle{width:5,height:1};

    //     assert!(!smaller.can_hold(&larger));
    // }
    // #[test]
    // fn it_adds_two(){
    //     assert_eq!(4,add_two(2));
    // }
    // #[test]
    // fn greeting_contains_name(){
    //     let result = greeting("Carol");
    //     assert!(result.contains("Carol"),"Greeting did not contain name, value was '{}'",result);
    // }
    //一个带有特定错误信息的panic!条件的测试
    // #[test]
    // #[should_panic(expected = "Guess value must be less than or equal to 100")]
    // fn greater_than_100(){
    //     Guess::new(200);
    // }
    //并行或连续的运行测试
    //当允许多个测试时,Rust默认使用线程来并行允许。
    //这意味着测试会更快地允许完毕
    //因为测试同时运行,确保它们不能互相依赖环境和状态
    //如果你不希望测试并行运行,或者想要精确地控制线程的数量
    //可以传递--test-threads参数和希望使用线程的数量
    //cargo test -- --test-threads=1

    //展示函数输出
    //默认情况下,当测试通过时,Rust的测试库会截获打印到标准输出的所有内容
    // #[test]
    // fn this_test_will_pass(){
    //     let value = prints_and_returns_10(4);
    //     assert_eq!(10,value);
    // }
    // #[test]
    // fn this_test_will_fail(){
    //     let value = prints_and_returns_10(8);
    //     assert_eq!(5,value);
    // }
    //如果你希望也能看到通过的测试中打印的值,可以通过在末尾增加--show-output参数来告知Rust显示通过测试的输出:
    //cargo test -- --show-output


    //三个不同名称的测试
    #[test]
    fn add_two_and_two(){
        assert_eq!(4,add_two(2));
    }

    #[test]
    fn add_three_and_two(){
        assert_eq!(5,add_two(3));
    }

    #[test]
    fn one_hundred(){
        assert_eq!(102,add_two(100));
    }

    #[test]
    fn it_works(){
        assert_eq!(2+2,4);
    }

    #[test]
    #[ignore]
    fn expensive_test(){
        //需要运行一个小时的代码
    }
    //运行单个测试:可以向cargo test传递任意测试的名称来只运行这个测试
    //例如:cargo test one_hundred
    //摘要行的结尾显示了 2 filtered out 表明还存在比本次所运行的测试更多的测试被过滤掉了。

    //过滤运行多个测试
    //我们可以指定部分测试的名称,任何名称匹配这个名称的测试会被运行
    //例如:cargo test add
    //会运行所有名字中带有add的测试

    //***同时注意测试所在的模块也是测试名称的一部分，所以可以通过模块名来运行一个模块中的所有测试。***

    //忽略某些测试
    //使用ignore属性来标记耗时的测试并排除他们
    //如果我们只希望运行被忽略的测试,可以使用cargo test -- --ignored
}
