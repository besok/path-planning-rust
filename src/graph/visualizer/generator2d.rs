use kiss3d::{window::{Window, State}, nalgebra::Translation2};


pub fn generate_canvas(title: &str ) -> Window{
    let window = Window::new(title);
    window
}


#[derive(Default)]
struct TestState{
    x_cord:usize,
    y_cord:usize,

    rad:usize,
    left:bool,
}

impl State for TestState {
    fn step(&mut self, w: &mut Window) {
        let mut c = w.add_circle(self.rad as f32);
        if self.left{
            c.append_translation(&Translation2::new((self.x_cord + self.rad )as f32, self.y_cord as f32));
        }else {
            c.append_translation(&Translation2::new(self.x_cord as f32, (self.y_cord +  2* self.rad) as f32));
        }
        self.left = !self.left;
        self.x_cord = self.x_cord + self.rad;
        self.y_cord = self.y_cord + self.rad;
    }
}


#[cfg(test)]
mod tests{
    use kiss3d::{nalgebra::{Translation2, Point2, Point3}, light::Light, window::Window, text::Font};

    use super::{generate_canvas, TestState};

    #[test]
    fn test(){

        let mut test_state = TestState::default();

        test_state.rad = 10;

        let mut canvas = generate_canvas("title");

         canvas.render_loop(test_state) ;

    }

    #[test]
    fn test_templ() {
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