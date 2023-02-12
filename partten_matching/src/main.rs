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
    test13();
}

// 修复错误，尽量少地修改代码
// 不要移除任何代码行
fn test13() {
    let mut v = String::from("hello,");
    let r = &mut v;

    match r {
        mut value => value.push_str(" world!") 
    }
    println!("The value of v is {}", v);
}

// 让代码工作，必须使用 `split`
fn test12() {
    let num = Some(4);
    let split = 5;
    match num {
        Some(x) if x < split => assert!(x < split),
        Some(x) => assert!(x >= split),
        None => (),
    }
}

struct Point {
    x: i32,
    y: i32,
}

// @ 操作符可以让我们将一个与模式相匹配的值绑定到新的变量上
// 当你既想要限定分支范围，又想要使用分支的变量时，就可以用 @ 来绑定到一个新的变量上，实现想要的功能。
fn test11() {
    // 填空，让 p 匹配第二个分支
    let p = Point { x: 2, y: 10 };

    match p {
        // 只有x可以直接输出
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        // 第二个分支
        Point { x: 0..=5, y: y2@ (10 | 20 | 30) } => println!("On the y axis at {}", y2),
        // 这里很神奇，为啥不能直接输出y
        // Point { x: 0..=5, y: (10 | 20 | 30) } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}

fn test10(){
    match_number(5);
}

// 使用 | 可以匹配多个值, 而使用 ..= 可以匹配一个闭区间的数值序列
fn match_number(n: i32) {
    match n {
        // 匹配一个单独的值
        1 => println!("One!"),
        // 使用 `|` 填空，不要使用 `..` 或 `..=`
        2 | 3 | 4 | 5 => println!("match 2 -> 5"),
        // 匹配一个闭区间的数值序列
        6..=10 => {
            println!("match 6 -> 10")
        },
        _ => {
            println!("match 11 -> +infinite")
        }
    }
}

// 无论是 match 还是 if let，他们都可以在模式匹配时覆盖掉老的值，绑定新的值:
fn test9() {
    let age = Some(30);
    if let Some(age) = age { // 创建一个新的变量，该变量与之前的 `age` 变量同名
        println!("匹配到的age是 {}", age);
        assert_eq!(Some(age), Some(30));
    } // 新的 `age` 变量在这里超出作用域

    println!("匹配后age是 {:?}", age);

    match age {
        // `match` 也能实现变量遮蔽
        Some(age) =>  println!("age 是一个新的变量，它的值是 {}",age),
        _ => ()
    }
}


enum Foo2 {
    Bar,
    Baz,
    Qux(u32)
}

fn test8() {
    let a = Foo2::Qux(10);

    // 移除以下代码，使用 `match` 代替
    if let Foo2::Bar = a {
        println!("match foo::bar")
    } else if let Foo2::Baz = a {
        println!("match foo::baz")
    } else {
        println!("match others")
    }

    match a {
        Foo2::Bar => {
            println!("match foo::bar");
        },
        Foo2::Baz => {
            println!("match foo::baz");
        },
        _ => {
            println!("match others");
        }
    }
}

enum Foo {
    Bar(u8)
}

fn test7() {
    let a = Foo::Bar(1);

    if let Foo::Bar(i) = a{
        println!("foobar 持有的值是: {}", i);
    }
}

// 当你只要匹配一个条件，且忽略其他条件时就用 if let ，否则都用 match。
fn test6() {
    let o = Some(7);

    // 移除整个 `match` 语句块，使用 `if let` 替代
    match o {
        Some(i) => {
            println!("This is a really long string and `{:?}`", i);
        }
        _ => {}
    };

    // 跟上边等价
    if let Some(i) = o{
        println!("This is a really long string and `{:?}`", i);
    }
}

enum MyEnum {
    Foo,
    Bar
}

fn test5() {
    let mut count = 0;

    let v = vec![MyEnum::Foo,MyEnum::Bar,MyEnum::Foo];
    for e in v {
        if matches!(e, MyEnum::Foo) { 
            count += 1;
        }
    }

    assert_eq!(count, 2);
}

fn test4() {
    let alphabets = ['a', 'E', 'Z', '0', 'x', '9' , 'Y'];

    // 使用 `matches` 填空
    for ab in alphabets {
        assert!(matches!(ab, 'a'..='z'|'A'..='Z'|'0'..='9'));
    }
} 

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// 使用 match 匹配出枚举成员持有的值
fn test3() {
    let msgs = [
        Message::Quit,
        Message::Move{x:1, y:3},
        Message::ChangeColor(255,255,0)
    ];

    for msg in msgs {
        show_message(msg)
    }
}

fn show_message(msg: Message) {
    match msg {
        Message::Move{x: a, y: b} => { // 这里匹配 Message::Move
            assert_eq!(a, 1);
            assert_eq!(b, 3);
        },
        Message::ChangeColor(_, g, b) => {
            assert_eq!(g, 255);
            assert_eq!(b, 0);
        },
        _ => println!("no data in these variants")
    }
}

fn test2() {
    let boolean = true;

    // match 是一个表达式，因此可以用在赋值语句中
    // boolean = true => binary = 1
    // boolean = false => binary = 0
    let binary = match boolean {
        true => 1,
        false => 0
    };

    assert_eq!(binary, 1);
}

enum Direction {
    East,
    West,
    North,
    South,
}

fn test1() {
    let dire = Direction::South;
    match dire {
        Direction::East => println!("East"),
        Direction::South | Direction::North  => { // 在这里匹配 South 或 North
            println!("South or North");
        },
        Direction::West => println!("West"),
    };
}
