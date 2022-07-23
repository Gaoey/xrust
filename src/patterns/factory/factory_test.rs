#[cfg(test)]
mod test {
    use crate::patterns::factory::factory::{ShapeFactory, ShapeType};

    #[test]
    fn test_factory() {
        let shape = ShapeFactory::new_shape(&ShapeType::Circle);
        shape.draw(); // output: draw a circle!

        let shape = ShapeFactory::new_shape(&ShapeType::Rectangle);
        shape.draw(); // output: draw a rectangle!
    }
}
