fn main() {
    // test1();
    // test2();
    // test3();
    // test4();
    // test5();
    // test6();
    // test7();
    test8();
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
