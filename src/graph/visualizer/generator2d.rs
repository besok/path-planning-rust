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
    use super::{generate_canvas, TestState};

    #[test]
    fn test(){

        let mut test_state = TestState::default();

        test_state.rad = 10;

        let mut canvas = generate_canvas("title");

         canvas.render_loop(test_state) ;

    }
}