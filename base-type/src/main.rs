fn main() {
    //number_type();
    //bitwise_op();
    //range();
    char_type();
    // bool_type();
    // 表达式测试,  表达式不能包含分号。这一点非常重要，一旦你在表达式后加上分号，它就会变成一条语句
    let y = {
        let x = 1;
        x + 2
    };
    println!("表达式求值: {}", y);

    //dead_end();
    wrapping_add();
    // 无返回值函数，默认返回()
    assert_eq!((), wrapping_add());

    println!("The result of 3 + 2 = {}", add(3, 2));
}

// Rust 是强类型语言，因此需要你为每一个函数参数都标识出它的具体类型
fn add(i: i32, j: i32) -> i32{
    i + j
}

fn wrapping_add(){
    let a:u8 = 255;
    let b = a.wrapping_add(20);
    // 19
    println!("The value of 255:u8 + 20 is {}", b);
    assert_eq!(19, b);
}

// 语句默认返回(), 但是这里类型是!, 用 ! 作函数返回类型的时候，表示该函数永不返回( diverge function )
fn dead_end() -> !{
    panic!("累了, 毁灭吧!!");
}

fn bool_type(){
    let a = true;
    if a {
        println!("a is true");
    }
}

//Rust 的字符只能用 '' 来表示， "" 是留给字符串的
fn char_type(){
    let x = '中';
    println!("字符'中'占用了{}字节的内存大小",std::mem::size_of_val(&x));

    // 输出你我之间间隔字符, 好几千个字符
    for i in '你'..='我' {
        println!("{}", i);
    }
}

// 序列
fn range(){
    println!("数字序列测试");
    for i in 1..=5{
        println!("i is {}", i);
    }

    println!("字符序列测试");
    for i in 'a'..='z'{
        println!("i is {}", i);
    }
}

// 位运算操作
fn bitwise_op(){

    // 2进制 00000010
    let a:i32=2;
    // 2进制 00000011
    let b:i32=3;

    //& 位与	相同位置均为1时则为1，否则为0
    println!("(a & b) value is {}", (a & b));
    //| 位或	相同位置只要有1时则为1，否则为0
    println!("(a | b) value is {}", (a | b));
    //^ 异或	相同位置不相同则为1，相同则为0
    println!("(a ^ b) value is {}", (a ^ b));
    // ! 位非	把位中的0和1相互取反，即0置为1，1置为0
    println!("!b value is {}", !b);
    //<< 左移	所有位向左移动指定位数，右位补零
    println!("(a << b) value is {}", (a << b));
    //>> 右移	所有位向右移动指定位数，左位补零
    println!("(a >> b) value is {}", (a >> b));

    let mut a = a;
    a <<= b;
    println!("a is {}", a);
}

// 基础数字类型

//数值类型: 有符号整数 (i8, i16, i32, i64, isize)、 无符号整数 (u8, u16, u32, u64, usize) 、浮点数 (f32, f64)、以及有理数、复数
//字符串：字符串字面量和字符串切片 &str
//布尔类型： true和false
//字符类型: 表示单个 Unicode 字符，存储为 4 个字节
//单元类型: 即 () ，其唯一的值也是 ()
fn number_type(){
    // 编译器自动推导, 默认i32
    let twenty = 20;
    let twenty_one:i32 = 21;
    let twenty_two = 22i32;

    let addition = twenty + twenty_one + twenty_two;
    println!("{} + {} + {} = {}", twenty, twenty_one, twenty_two, addition);

    let one_million:i64 = 1_000_000;
    // 2次方
    println!("{}", one_million.pow(2));

    // 定义f32数组, 第一个值自动推导为f32
    // 浮点数默认f64
    let forty_two = [
        42.0,
        42f32,
        42.0_f32
    ];
    // 保留两位小数
    println!("{:.2}", forty_two[0]);
}
