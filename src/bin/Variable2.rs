/*
可变绑定
rust 在声明变量时，在变量前面加入 mut 关键字，变量就会成为可变绑定的变量。
 */
fn main() {
    let mut a: f64 = 1.0;
    let b = 2.0f32;

    //改变 a 的绑定
    a = 2.0;
    println!("{:?}", a);

    //重新绑定为不可变
    let a = a;

    //不能赋值
    //a = 3.0;

    //类型不匹配
    //assert_eq!(a, b);
}