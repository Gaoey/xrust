pub trait Shape {
    fn draw(&self);
}

pub enum ShapeType {
    Rectangle,
    Circle,
}

pub struct Rectangle {}

impl Shape for Rectangle {
    fn draw(&self) {
        println!("draw a rectangle!");
    }
}

pub struct Circle {}

impl Shape for Circle {
    fn draw(&self) {
        println!("draw a circle!");
    }
}

pub struct ShapeFactory;
impl ShapeFactory {
    pub fn new_shape(s: &ShapeType) -> Box<dyn Shape> {
        match s {
            ShapeType::Circle => Box::new(Circle {}),
            ShapeType::Rectangle => Box::new(Rectangle {}),
        }
    }
}
