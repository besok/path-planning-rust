#[cfg(test)]
mod test {
    use kiss3d::light::Light;
    use kiss3d::nalgebra::{Point2, Point3, Translation2, UnitQuaternion, Vector3};
    use kiss3d::text::Font;
    use kiss3d::window::Window;

    #[test]
    fn test() {
        let mut window = Window::new("Test");
        let mut c1 = window.add_circle(100.0);
        let mut c2 = window.add_circle(50.0);

        c1.set_color(1.0, 0.0, 0.0);
        c1.append_translation(&Translation2::new(200.0, 0.0));
        c2.set_color(1.0, 1.0, 0.0);

        window.set_light(Light::StickToCamera);

        let font = Font::default();

        while window.render() {
            window.draw_text(
                "Hello birds!",
                &Point2::origin(),
                120.0,
                &font,
                &Point3::new(0.0, 1.0, 1.0),
            );


            window.draw_text(
                "text",
                &Point2::new(0.0, 120.0),
                60.0,
                &font,
                &Point3::new(1.0, 1.0, 0.0),
            );
        }
    }
}
