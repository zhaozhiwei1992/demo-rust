// 利用特征对象设计一个屏幕渲染小功能, 类似java中接口实现类的思想

// 设计draw方法
pub trait Draw {
  fn draw(&self);
}

struct Button {
  width: u32,
  height: u32,
  label: String
}

impl Draw for Button {
  fn draw(&self){
    println!("按钮绘制, 长:{}, 宽: {}, 标题: {}", self.width, self.height, self.label);
  } 
}

struct SelectBox {
  width: u32,
  height: u32,
  label: String
}

impl Draw for SelectBox {
  fn draw(&self){
    println!("下拉框绘制, 长:{}, 宽: {}, 标题: {}", self.width, self.height, self.label);
  } 
}

// 屏幕渲染, 多个组件
pub struct Screen {
  pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
  fn show(&self) {
    for component in self.components.iter() {
      component.draw();
    }
  }

  pub fn run(&self){
    let screen = Screen{
        components: vec![
            Box::new(Button {
              width: 18,
              height: 4,
              label: String::from("新增按钮")
            }),
            Box::new(SelectBox{
              width: 18,
              height: 4,
              label: String::from("一个下拉框")

            })
        ]
    };
    screen.show();
  }
}