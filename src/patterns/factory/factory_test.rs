#[cfg(test)]
mod test {
    use crate::patterns::factory::factory::{ShapeFactory, ShapeType};

    #[test]
    fn test_factory() {
        let shape = ShapeFactory::new_shape(&ShapeType::Circle);
        assert_eq!(shape.draw(), "draw a circle!");

        let shape = ShapeFactory::new_shape(&ShapeType::Rectangle);
        assert_eq!(shape.draw(), "draw a rectangle!");
    }
}
