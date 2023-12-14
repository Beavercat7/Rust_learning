//测试的组织结构

//单元测试

//测试模块和#[cfg(test)]
//测试模块的 #[cfg(test)] 标注告诉 Rust 只在执行 cargo test 时才编译和运行测试代码，而在运行 cargo build 时不这么做。
pub fn add_two(a:i32) -> i32{
    internal_adder(a,2)
}

fn internal_adder(a:i32,b:i32) -> i32{
    a+b
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn internal(){
        assert_eq!(4,internal_adder(2,2));
    }
    
    
    //集成测试
    //为了创建集成测试,你需要先创建一个tests目录
}

