// 定义struct
struct Struct {
    e: i32
}

const MAX_POINTS:f32 = 100_000.00;

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

    let (a, b, c, d, e);
    //a:1, b:2
    (a, b) = (1, 2);
    // c:1, d:4  ..表示中间任意长度吧
    // _ 代表匹配一个值，但是我们不关心具体的值是什么，因此没有使用一个变量名而是使用了 _
    [c, .., d, _] = [1, 2, 3, 4, 5];
    Struct {e, ..} = Struct {e:5};
    assert_eq!([1, 2, 1, 4, 5], [a, b, c, d, e]);

    //常量，永远不可变
    println!("The max_points is {}", MAX_POINTS);

    // 变量覆盖
    let x = x + 1;
    {
        let x = x * 2;
        // 14
        // println!("The value of x is {}", x);
        assert_eq!(14, x);
    }
    // 7
    // println!("The value of x is {}", x);
    assert_eq!(7, x);

    let spaces = "     ";
    println!("空格字符串spaces {}", spaces);
    // 这种结构是允许的，因为第一个 spaces 变量是一个字符串类型，第二个 spaces 变量是一个全新的变量且和第一个具有相同的变量名，且是一个数值类型。所以变量遮蔽可以帮我们节省些脑细胞，不用去想如 spaces_str 和 spaces_num 此类的变量名；相反我们可以重复使用更简单的 spaces 变量名
    // 必须用let
    let spaces = spaces.len();
    println!("字符串spaces长度 {}", spaces);
}
