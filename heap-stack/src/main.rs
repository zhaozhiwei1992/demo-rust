// 谨记以下规则：
//Rust 中每一个值都被一个变量所拥有，该变量被称为值的所有者
//    一个值同时只能被一个变量所拥有，或者说一个值只能拥有一个所有者
//    当所有者(变量)离开作用域范围时，这个值将被丢弃(drop)

fn main() {
    // 变量作用域
    {                      // s 在这里无效，它尚未声明
        let s = "hello";   // 从此处起，s 是有效的

        // 使用 s
    }                      // 此作用域已结束，s不再有效
    // println!("The value of s {}", s);

    let s1 = String::from("hello");
    println!("The value of s1 {}", s1);

    // 看着像是浅copy, 实际上， s1从此失效, rust原则(一个值同时只能被一个变量所拥有，或者说一个值只能拥有一个所有者)
    // 此时s2才是heap所有者
    let s2 = s1;
    //println!("The value of s1 {}", s1);
    println!("The value of s2 {}", s2);

    // 这里跟用String不同，这种表示x直接应用到字符串，相当与是x = "hello, world" 固定值, 栈
    let x: &str = "hello, world";
    println!("The value of x {}", x);
    let y = x;
    println!("The value of x {}", x);

    // 克隆测试
    testClone();

    // 测试函数传值影响
    testMethodParam();

}

//将值传递给函数，一样会发生 移动 或者 复制，就跟 let 语句一样
fn testMethodParam(){
    let x = String::from("hello");
    takes_ownership(x);
    // 下述输出会报错，所有权失去
    // println!("The value of x {}", x);

    // 基础类型, 不会丢失所有权, 基础类型基本存储在栈中
    let y = 10;
    make_copy(y);
    println!("The value of y {}", y);
}

fn make_copy(a_integer:i32){
    println!("The value of a_integer {}", a_integer);
}

// 如果是个引用类型对象传入, 调用这方法以后，所有权也会失去
fn takes_ownership(some_string:String){
    println!("The value of some_string {}", some_string);
}

fn testClone(){
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}
