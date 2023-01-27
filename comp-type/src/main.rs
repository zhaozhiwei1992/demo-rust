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
    tuple_test18();
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
