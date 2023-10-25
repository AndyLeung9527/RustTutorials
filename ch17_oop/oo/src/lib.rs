//面向对象的程序是由对象组成的。一个对象包含数据和操作这些数据的过程。这些过程通常被称为方法或操作
//封装：对象的实现细节不能被使用对象的代码获取到，唯一与对象交互的方式是通过对象提供的公有API，封装使得改变和重构对象的内部时无需改变使用对象的代码

//结构体自身被标记为pub，内部字段是私有的
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}
//通过在结构体上实现公有的add、remove和average方法使得外部可以操作结构体
impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }
    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }
    pub fn average(&self) -> f64 {
        self.average
    }
    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}
