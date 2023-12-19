//通过RefCell<T>在运行时检查借用规则
/*
回忆一下第4章所学的借用规则
1.在任意给定时刻,只能拥有一个可变引用或任意数量的不可变引用之一
2.引用必须总是有效的。
*/
//在不可变值内部改变值就是内部可变性模式
fn main() {
    println!("Hello, world!");
}