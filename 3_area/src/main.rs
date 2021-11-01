//形状trait
trait ShapeTrait {
    fn get_shape() -> Shape;
    fn get_value(self) -> u32;
}

//形状
enum Shape {
    Square,
    Circle,
}

//圆
struct CircleStruct {
    radius: u32,
}

impl ShapeTrait for CircleStruct {
    fn get_shape() -> Shape {
        Shape::Circle
    }
    fn get_value(self) -> u32 {
        self.radius
    }
}

struct SquareStruct {
    side_length: u32,
}

//实现trait
impl ShapeTrait for SquareStruct {
    fn get_shape() -> Shape {
        Shape::Square
    }
    fn get_value(self) -> u32 {
        self.side_length
    }
}

//进行范型 参数约束
fn get_area<T: ShapeTrait>(object: T) -> f64 {
    let value = object.get_value()as f64;
    match <T as ShapeTrait>::get_shape() {
        Shape::Square => value.powi(2),
        //与 浮点数运算，统一为浮点数
        Shape::Circle => 3.14159 * value.powi(2),
    }
}

fn main() {

    //创建实例
    //半径10的 圆
    let circle = CircleStruct { radius: 10 };
    //边长10的 正方形
    let square = SquareStruct { side_length: 10 };

    //打印结果 调用 get_area(XXX)
    println!(
        "圆的面积是 ：{}\n正方形面积为：{}",
        get_area(circle),
        get_area(square)
    );
}
