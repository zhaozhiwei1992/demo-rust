// 使用struct实现复杂数组 (推荐)
trait IpAddr {
    fn display(&self);
}

#[derive(Debug)]
struct v4 {
    ip: String,
}

impl IpAddr for v4 {
    fn display(&self) {
        println!("ipv4 {:?}", self.ip);
    }
}

#[derive(Debug)]
struct v6 {
    ip: String,
}

impl IpAddr for v6 {
    fn display(&self) {
        println!("ipv6 {:?}", self.ip);
    }
}

pub fn test1() {
    let v: Vec<Box<dyn IpAddr>> = vec![
        Box::new(v4 {
            ip: "127.0.0.1".to_string(),
        }),
        Box::new(v6 {
            ip: "::1".to_string(),
        }),
    ];

    for ip in v {
        ip.display()
    }
}

#[derive(Debug)]
struct Person{
  name: String,
  age: u8,
}

impl Person {
  fn new(name: String, age: u8) -> Person{
    // return Person{name: name, age: age};
    return Person{name, age};
  }
}

pub fn test2(){
  let mut person = vec![
    Person::new("zhangsan".to_string(), 18),
    Person::new("lisi".to_string(), 20),
    Person::new("wangwu".to_string(), 19),
  ];

  println!("排序前");
  for p in &person{
    println!("{:?}", p);
  }
  // 倒序
  // 这里为啥要用&a, 感觉是比较的时候怕所有权变了, 跟内部实现有关
  person.sort_unstable_by(|a, b| b.age.cmp(&a.age));
  println!("排序后");
  for p in &person{
    println!("{:?}", p);
  }
}