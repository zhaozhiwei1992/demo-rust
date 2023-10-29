pub trait Summary {
  fn summarize(&self) -> String;

  // 特征中实现默认方法, 类似j8接口中的default method
  fn summarize2(&self) -> String {
    return String::from("default msg");
  }

}

// 类似java中定义一个类
pub struct Post {
  pub title: String, // 标题
  pub author: String, // 作者
  pub content: String // 内容
}

// 给上述类实现些接口, 并创建对象
impl Summary for Post {
  fn summarize(&self) -> String{
    return format!("文章标题是: {}, 作者是: {}", self.title, self.author);
  }
}

pub struct Weibo {
  pub username: String,
  pub content: String
}

impl Summary for Weibo {
  fn summarize(&self) -> String {
    return format!("{} 发表了微博,内容是: {}", self.username, self.content);
  }
}

// pub fn notify(item: &impl Summary) {
//     println!("{}", item.summarize());
// }

// 与上述notify写法等价, 这种泛型写法更灵活, 如果有多个item参数, 可限制多个参数都是基于T (推荐下述写法)
pub fn notify<T: Summary>(item: &T) {
    println!("{}", item.summarize());
}

// 获取集合中最大属性, 并返回
// 需要实现集合属性比较特征, 并且涉及借用
pub fn largest<T: Copy + PartialOrd>(list: &[T]) -> T {
  let mut largest = list[0];
  for &item in list.iter() {
    if item > largest {
      largest = item;
    }
  }
  return largest;
}