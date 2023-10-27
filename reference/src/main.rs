fn main() {

    test1();

    test2();

    // 引用并且修改内容
    testRefModify();

    // dangle();
}

// 测试引用和解引用
fn test1(){
    let s1 = String::from("hello");
    // 该方式, 将s1所有权交给s2, 后续使用s1就会报错
    //let s2 = s1;
    //println!("s1 = {}", s1);

    // 借用, 将引用给s2
    let s2 = &s1;
    // *s2解引用拿到s1, 两个相同, s1 
    assert_eq!(s1, *s2);
    println!("s1 = {}", s1);
}

// 测试给函数传递引用, 防止所有权变化
fn test2(){
    let s1 = String::from("hello");
    let len = cal_length(&s1);
    // 上述方式不会改变所有权，s1仍可正常使用
    println!("{} length {}", s1, len);
}

fn cal_length(s: &String) -> usize{
    // 用函数为啥就不用 *s.len(), 应该会自动解包
    // 如果直接访问, 如下, 需要*s2解引用拿到s1
    // let s2 = &s1;
    // assert_eq!(s1, *s2);
   return s.len();
}

// 测试引用修改
fn testRefModify(){
    let mut s1 = String::from("");
    s1.push_str("hello, ");
    // &s1相当与s1的一个软链接
    // 注意修改的话需要mut, 变量定义, 实参, 形参
    change_str(&mut s1);
    println!("修改后: s1 = {}", s1);
}

fn change_str(s:&mut String) {
    s.push_str("world");
}

// 悬垂引用也叫做悬垂指针，意思为指针指向某个值后，这个值被释放掉了，而指针仍然存在，其指向的内存可能不存在任何值或已被其它变量重新使用
// rust直接运行报错，保证错误不会扩散开
fn dangle() -> &String{
    let s1 = String::from("");
    return &s1;
}
// 这里s1就会释放，返回的&s1就成了孤儿(悬锤引用)
