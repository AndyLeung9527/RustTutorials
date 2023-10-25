//trait对象执行动态分发，在运行时才使用trait对象中的指针来确定需要调用哪个方法
pub trait Draw {
    fn draw(&self);
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}
impl Draw for Button {
    fn draw(&self) {}
}

pub struct Screen {
    //Box<dyn Draw>是一个trait对象，是Box中任何实现了Draw trait的类型，Vec具体类型可以多种
    //若使用泛型的话，则会使Vec具体类型必须一致，只有一种
    pub components: Vec<Box<dyn Draw>>,
}
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
