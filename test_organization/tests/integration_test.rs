use test_organization;
mod common;
//一个adder crate中函数的集成测试
#[test]
fn it_adds_two(){
    common::setup();
    assert_eq!(4,test_organization::add_two(2));
}
//与单元测试不同，我们需要在文件顶部添加 use adder。这是因为每一个 tests 目录中的测试文件都是完全独立的 crate，所以需要在每一个文件中导入库
/*
我们已经知道，单元测试函数越多，单元测试部分的结果行就会越多。同样的，在集成文件中增加的测试函数越多，也会在对应的测试结果部分增加越多的结果行。每一个集成测试文件有对应的测试结果部分，所以如果在 tests 目录中增加更多文件，测试结果中就会有更多集成测试结果部分。
*/
//cargo test 的 --test 后跟文件的名称来运行某个特定集成测试文件中的所有测试
//例如:cargo test --test integration_test
//这个命令只运行了 tests 目录中我们指定的文件 integration_test.rs 中的测试。

//集成测试中的子模块
