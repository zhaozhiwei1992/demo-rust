fn main() {
    // test1();
    // test2();
    // test3();
    // test4();
    // test5();
    // test6();
    // test7();
    // test8();
    // test9();
    // test10();
    // test11();
    // test12();

    // tuple测试
    // tuple_test13();
    // tuple_test14();
    // tuple_test15();
    // tuple_test16();
    // tuple_test17();
    // tuple_test18();

    // struct测试
    // struct_test1();
    // struct_test2();
    // struct_test3();
    // struct_test4();
    // struct_test5();
    // struct_test6();

    // 枚举测试
    // enum_test1();
    // enum_test2();
    // enum_test3();
    // enum_test4();
    // enum_test5();
    enum_test6();

    // 数组测试
}

use crate::List::*;

enum List {
    // Cons: 链表中包含有值的节点，节点是元组类型，第一个元素是节点的值，第二个元素是指向下一个节点的指针
    Cons(u32, Box<List>),
    // Nil: 链表中的最后一个节点，用于说明链表的结束
    Nil,
}

// 为枚举实现一些方法
impl List {
    // 创建空的链表
    fn new() -> List {
        // 因为没有节点，所以直接返回 Nil 节点
        // 枚举成员 Nil 的类型是 List
        Nil
    }

    // 在老的链表前面新增一个节点，并返回新的链表
    fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }

    // 返回链表的长度
    fn len(&self) -> u32 {
        match *self {
            // 这里我们不能拿走 tail 的所有权，因此需要获取它的引用
            Cons(_, ref tail) => 1 + tail.len(),
            // 空链表的长度为 0
            Nil => 0
        }
    }

    // 返回链表的字符串表现形式，用于打印输出
    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                // 递归生成字符串
                format!("{}, {}", head, tail.stringify())
            },
            Nil => {
                format!("Nil")
            },
        }
    }
}

fn enum_test6() {
    // 创建一个新的链表(也是空的)
    let mut list = List::new();

    // 添加一些元素
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    // 打印列表的当前状态
    println!("链表的长度是: {}", list.len());
    println!("{}", list.stringify());
}

fn enum_test5() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    if let Some(n) = six {
        println!("{}", n)
    }

    panic!("不要让这行代码运行！");
} 

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}


// 枚举成员可以持有各种类型的值
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn enum_test4() {
    let msgs: [Message; 3] = [
        Message::Quit,
        Message::Move{x:1, y:3},
        Message::ChangeColor(255,255,0)
    ];

    for msg in msgs {
        show_message(msg)
    }
} 

fn show_message(msg: Message) {
    println!("{:?}", msg);
}

fn enum_test3(){
    let msg = Message::Move{x: 1, y: 1};

    if let Message::Move{x: a, y: b} = msg {
        assert_eq!(a, b);
    } else {
        panic!("不要让这行代码运行！");
    }
}

fn enum_test2() {
    let msg1 = Message::Move{x:1, y:2}; // 使用x = 1, y = 2 来初始化
    let msg2 = Message::Write("hello, world!".to_string()); // 使用 "hello, world!" 来初始化
} 

// 创建枚举时，可以使用显式的整数设定枚举成员的值。
enum Number {
    Zero = 0,
    One = 1,
    Two,
}

enum Number1 {
    Zero = 0,
    One = 1,
    Two,
}

// C语言风格的枚举定义
enum Number2 {
    Zero = 0,
    One = 1,
    Two = 2,
}


fn enum_test1() {
    // 通过 `as` 可以将枚举值强转为整数类型
    assert_eq!(Number::One as u8, Number1::One as u8);
    assert_eq!(Number1::One as u8, Number2::One as u8);
} 

// 使用结构体更新语法基于一个结构体实例来构造另一个
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
fn struct_test6() {
    let u1 = User {
        email: String::from("someone@example.com"),
        username: String::from("sunface"),
        active: true,
        sign_in_count: 1,
    };

    let u2 = set_email(u1);

    // 打印 debug 信息到标准错误输出 stderr
    dbg!(&u2);

    println!("{:?}", u2);
}

fn set_email(u: User) -> User {
    User {
        email: String::from("contact@im.dev"),
        //username: u.username,
        //active: u.active,
        //sign_in_count: u.sign_in_count
        // TypeScript 过来，肯定觉得啰嗦爆了：竟然手动把 user1 的三个字段逐个赋值给 user2，好在 Rust 为我们提供了 结构体更新语法
        ..u
    }
}


// 填空并修复错误
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
fn struct_test4() {
    let v = Point(0, 127, 255);
    check_color(v);
}

fn check_color(p: Point) {
    let Point(x, _, z) = p;
    assert_eq!(x, 0);
    assert_eq!(p.1, 127);
    assert_eq!(z, 255);
}

struct Unit;

trait SomeTrait {
    // ...定义一些行为
}

// 我们并不关心结构体中有什么数据( 字段 )，但我们关心它的行为。
// 使用没有任何字段的单元结构体，然后为它实现一些行为
impl SomeTrait for Unit {  }

fn struct_test3() {
    let u = Unit;
    do_something_with_unit(u);
}

// 填空，让代码工作
fn do_something_with_unit(u: Unit) {   }

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
    hobby: String
}

fn struct_test2() {
    let age = 30;
    //  Rust 不允许我们将结构体的某个字段专门指定为可变的, 只能定义整个都是可变
    let mut p = Person {
        name: String::from("sunface"),
        age: 18,
        hobby: String::from("hobby")
    };

    p.age = 18;
    p.name = String::from("sunfei");

    println!("{:?}", p);
} 

fn struct_test5() {
    println!("{:?}", build_person(String::from("zhangsan"), 18));
}

// 结构体字段初始化缩略语法可以减少一些重复代码
// 类似es6 相同名字可以直接使用
fn build_person(name: String, age: u8) -> Person {
    // age, name和参数相同, 可直接引用
    Person {
        age,
        name,
        hobby: String::from("hobby")
    }
}

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

fn struct_test1(){
    let f1 = File {
        name: String::from("f1.txt"),
        data: Vec::new(),
    };

    // 这里使用&f1是因为如果直接使用f1,就会将所有权交出去，导致后续f1直接失效
    let f1_name = &f1.name;
    let f1_length = &f1.data.len();

    println!("{:?}", f1);
    println!("{} is {} bytes long", f1_name, f1_length);
}

fn tuple_test18(){
    // 元组可以用于函数的参数和返回值
    // 填空，需要稍微计算下
    let (x, y) = sum_multiply((2, 3));

    assert_eq!(x, 5);
    assert_eq!(y, 6);
}

fn sum_multiply(nums: (i32, i32)) -> (i32, i32) {
    (nums.0 + nums.1, nums.0 * nums.1)
}

fn tuple_test17(){
    //解构式赋值
    let (x, y, z);

    // 填空
    (y, z, x) = (1, 2, 3);
    assert_eq!(x, 3);
    assert_eq!(y, 1);
    assert_eq!(z, 2);
}

fn tuple_test16(){

    // 使用模式匹配来解构元组
    let tup = (1, 6.4, "hello");
    // 填空
    let (x, z, y) = tup;

    assert_eq!(x, 1);
    assert_eq!(y, "hello");
    assert_eq!(z, 6.4);
}

fn tuple_test15(){
    // 过长的元组无法被打印输出
    // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    //? 最多12个?
    let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
    println!("too long tuple: {:?}", too_long_tuple);
}

fn tuple_test14(){
    // 可以使用索引来获取元组的成员
    let t = ("i", "am", "sunface");
    assert_eq!(t.2, "sunface"); 
}

fn tuple_test13(){
    // 元组中的元素可以是不同的类型。元组的类型签名是 (T1, T2, ...), 这里 T1, T2 是相对应的元组成员的类型.
    let _t0: (u8,i16) = (0, -1);
    // 元组的成员还可以是一个元组
    let _t1: (u8, (i16, u32)) = (0, (-1, 1));
    // 填空让代码工作
    // let t: (u8, __, i64, __, __) = (1u8, 2u16, 3i64, "hello", String::from(", world"));
    let t: (u8, u16, i64, &str, String) = (1u8, 2u16, 3i64, "hello", String::from(", world"));
}

fn test12(){
    // 填空，打印出 "你好，世界" 中的每一个字符
    for c in "你好，世界".chars() {
        println!("{}", c)
    }
}

fn test11(){
    // 你无法通过索引的方式去访问字符串中的某个字符，但是可以使用切片的方式 &s1[start..end] ，但是start 和 end 必须准确落在字符的边界处.
    let s1 = String::from("hi,中国");
    //let h = s1[0]; // 修改当前行来修复错误，提示: `h` 字符在 UTF-8 格式中只需要 1 个字节来表示
    let h = &s1[0..1];
    assert_eq!(h, "h");

    //let h1 = &s1[3..5];// 修改当前行来修复错误，提示: `中` 字符在 UTF-8 格式中需要 3 个字节来表示
    let h1 = &s1[3..6];
    assert_eq!(h1, "中");
}

fn test10(){
    // 想要一个非 UTF-8 形式的字符串吗(我们之前的 str, &str, String 都是 UTF-8 字符串) ? 可以试试字节字符串或者说字节数组:
     // 注意，这并不是 `&str` 类型了！
    let bytestring: &[u8; 21] = b"this is a byte string";


    // 字节数组没有实现 `Display` 特征，因此只能使用 `Debug` 的方式去打印
    println!("A byte string: {:?}", bytestring);

    // 字节数组也可以使用转义
    let escaped = b"\x52\x75\x73\x74 as bytes";
    // ...但是不支持 unicode 转义
    // let escaped = b"\u{211D} is not allowed";
    println!("Some escaped bytes: {:?}", escaped);


    // raw string
    let raw_bytestring = br"\u{211D} is not escaped here";
    println!("{:?}", raw_bytestring);

    // 将字节数组转成 `str` 类型可能会失败
    //if let Ok(my_str) = str::from_utf8(raw_bytestring) {
    //    println!("And the same as text: '{}'", my_str);
    //}

    let _quotes = br#"You can also use "fancier" formatting, \
                    like with normal raw strings"#;

    // 字节数组可以不是 UTF-8 格式
    let shift_jis = b"\x82\xe6\x82\xa8\x82\xb1\x82\xbb"; // "ようこそ" in SHIFT-JIS

    // 但是它们未必能转换成 `str` 类型
    //match str::from_utf8(shift_jis) {
    //    Ok(my_str) => println!("Conversion successful: '{}'", my_str),
    //    Err(e) => println!("Conversion failed: {:?}", e),
    //};
}

fn test9(){
    // 有时候需要转义的字符很多，我们会希望使用更方便的方式来书写字符串: raw string.
    let raw_str = "Escapes don't work here: \x3F \u{211D}";
    // 修改上面的行让代码工作
    assert_eq!(raw_str, "Escapes don't work here: ? ℝ");

    // 如果你希望在字符串中使用双引号，可以使用以下形式
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

    // 如果希望在字符串中使用 # 号，可以如下使用：
    let  delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", delimiter);

    // 填空
    let long_delimiter = r###"Hello, "##""###;
    assert_eq!(long_delimiter, "Hello, \"##\"")
}

fn test8() {
    // 你可以使用转义的方式来输出想要的字符，这里我们使用十六进制的值，例如 \x73 会被转义成小写字母 's'
    // 填空以输出 "I'm writing Rust"
    let byte_escape = "I'm writing Ru\x73t!";
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

    // 也可以使用 Unicode 形式的转义字符
    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

    println!("Unicode character {} (U+211D) is called {}",
             unicode_codepoint, character_name );

    // 还能使用 \ 来连接多行字符串
    let long_string = "String literals
                        can span multiple lines.
                        The linebreak and indentation here \
                        can be escaped too!";
    println!("{}", long_string);
}

//使用两种方法将 &str 转换成 String 类型
fn test7(){
    let s = "hello, world";

    // 方法1
    let sStr = String::from("hello, world");
    assert_eq!(sStr, s.to_string());
    assert_eq!(sStr, String::from(s));
}

// 你只能将 String 跟 &str 类型进行拼接，并且 String 的所有权在此过程中会被 move
fn test6(){
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; 
    assert_eq!(s3,"hello,world!");
    println!("{}",s3);
}

// 用 replace 方法来替换指定的子字符串
fn test5(){
    let s = String::from("I like dogs");
    // 以下方法会重新分配一块内存空间，然后将修改后的字符串存在这里
    let s1 = s.replace("dogs", "cats");

    assert_eq!(s1, "I like cats")
}

// 修复所有错误，并且不要新增代码行
fn test4(){
    let mut s = String::from("hello");
    s.push_str(",");
    s.push_str(" world");
    s += &"!".to_string();
    println!("{}", s)
}

fn test3(){
    let mut s = String::from("");
    s.push_str("hello, world");
    s.push_str("!");
    assert_eq!(s, "hello, world!");
}

// 如果要使用 str 类型，只能配合 Box。 & 可以用来将 Box<str> 转换为 &str 类型
fn test2(){
    let s: Box<str> = "hello, world".into();
    greetings(&s)
}

//fn greetings(s: Box<str>) {
//    println!("{}",s)
//}

fn greetings(s: &str) {
    println!("{}",s)
}

fn test1(){
    // 正常情况下我们无法使用 str 类型，但是可以使用 &str 来替代
    // 默认""hello, world"是&str类型
    // let s: str = "hello, world";
    // 修正:
    let s1: &str = "hello, world";
    println!("The value of s1 {}", s1);
}
