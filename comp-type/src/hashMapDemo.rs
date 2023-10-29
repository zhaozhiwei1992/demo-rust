use std::collections::HashMap;

pub fn test1(){
  // 这玩意儿好像不能放不同的类型, 说不定可以通过Box来搞了
  let mut map = HashMap::new();
  map.insert("name", "zhangsan");
  map.insert("age", "18");

  println!("{:?}", map);

  let age = map.get("age");
  println!("年龄: {:?}", age);

  let sex = map.entry("sex").or_insert("男");
  assert_eq!(*sex, "sex");
}

// 遍历数组生成map
pub fn test2(){
  // 方法1
  let fruit_list = vec![
    ("apple".to_string(), 20),
    ("orige".to_string(), 19),
    ("banana".to_string(), 10),
  ];

  let mut map = HashMap::new();
  for fruit in &fruit_list{
    map.insert(&fruit.0, fruit.1);
  }
  println!("水果数组 {:?}", fruit_list);
  println!("水果map {:?}", map);

  // 方法2, 类似java8的stream
  let map: HashMap<_, _> = fruit_list.into_iter().collect();
  println!("水果map {:?}", map);
}