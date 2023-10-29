#[derive(Debug)]
enum IpAddr {
  V4(String),
  V6(String),
}

// 动态数组存储枚举类型, 实现存储不同对象
pub fn test1() {
  //1. 创建枚举数组
  let v = vec![
    IpAddr::V4("127.0.0.1".to_string()),
    IpAddr::V6("::1".to_string())
  ];
  //2. 遍历输出
  for ip in v {
    println!("{:?}", ip);
  }
}

// 动态数组排序
pub fn test2(){
  let mut v = vec![6, 1, 2, 5,1, 6];
  println!("排序前:");
  for i in &v{
    println!("{i}");
  }
  // 这里要注意上诉循环&v, 否则所有权转移就没得了
  v.sort_unstable();
  println!("排序后:");
  for i in &v{
    println!("{i}");
  }

}
