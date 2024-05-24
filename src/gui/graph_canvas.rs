


struct GraphCanvas<'a> {
    data: &'a GraphData,
}

impl<'a> canvas::Program<Message> for GraphCanvas<'a> {
    fn draw(&self, bounds: iced::Rectangle, _cursor: canvas::Cursor) -> Vec<canvas::Geometry> {
        let canvas = canvas::Canvas::new()
            .fill(Color::WHITE)
            .stroke(Color::BLACK)
            .stroke_width(2);

        // Logic to draw your graph based on `self.data`

        vec![canvas.into_geometry()]
    }
}