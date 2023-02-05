#![allow(unused)]
fn main() {
    test10();
    // test9();
    // test8();
    // test7();
    // test6();
    // test5();
    // test4();
    // test3();
    // test2();
    // test1();
}

fn test10() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            // break 还可以直接返回
            break 20;
        }
    };

    assert_eq!(result, 20);
}

// 填空，不要修改其它代码
fn test9() {
    let mut count = 0u32;

    println!("Let's count until infinity!");

    // 无限循环
    loop {
        count += 1;

        if count == 3 {
            println!("three");

            // 跳过当此循环的剩余代码
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("OK, that's enough");

            break;
        }
    }

    assert_eq!(count, 5);
}

// 填空，不要修改其它代码
fn test8() {
    let mut n = 0;
    for i in 0..=100 {
        if n != 66 {
            n+=1;
            continue;
        }
        break;
    }

    assert_eq!(n, 66);
}


// 66时结束循环
fn test7() {
    let mut n = 0;
    for i in 0..=100 {
        if n == 66 {
            break;
        }
        n += 1;
    }

    assert_eq!(n, 66);
}


// 填空，让最后一行的  println! 工作 !
fn test6() {
    // 一个计数值
    let mut n = 1;

    // 当条件为真时，不停的循环
    while n <= 10 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }


        n = n + 1;
    }

    println!("n 的值是 {}, 循环结束",n);
}

fn test5() {
    let a = [4,3,2,1];

    // 通过索引和值的方式迭代数组 `a` 
    for (i,v) in a.iter().enumerate() {
        println!("第{}个元素是{}",i+1,v);
    }
}

// 不要新增或删除代码行
fn test4() {
    let names = [String::from("liming"),String::from("hanmeimei")];
    // for name in &names {
    // 注意move
    for name in &names {
        // do something with name...
    }

    println!("{:?}", names);

    let numbers = [1, 2, 3];
    // numbers中的元素实现了 Copy，因此无需转移所有权
    for n in numbers {
        // do something with name...

    }
    println!("{:?}", numbers);
}


fn test3() {
    for n in 1..=99 { // 修改此行，让代码工作
        // 修改循环范围, 不让if执行即可
        if n == 100 {
            panic!("NEVER LET THIS RUN")
        }
    }
}

// if/else 可以用作表达式来进行赋值
fn test2() {
    let n = 5;

    let big_n =
        if n < 10 && n > -10 {
            println!(" 数字太小，先增加 10 倍再说");

            10 * n
        } else {
            println!("数字太大，我们得让它减半");

            n / 2
        };

    println!("{} -> {}", n, big_n);
}

// 填空
fn test1() {
    let n = 5;

    if n < 0 {
        println!("{} is negative", n);
    } else if n > 0 {
        println!("{} is positive", n);
    } else {
        println!("{} is zero", n);
    }
} 

