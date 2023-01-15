fn main() {
    // ^^^^^ cannot assign twice to immutable variable
    //let x = 5;
    // 默认参数不可变, 需要制定mut来让参数可变
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);

    let _x = 5;
    // 如果定义变量不使用, 默认会提示warning, 可以变量前增加下划线
    let _y = 10;

    // 变量解构
    let (a, mut b):(bool, bool) = (true, false);
    println!("a is {:?}, b is {:?}", a, b);
    b = true;
    assert_eq!(a, b);
}
