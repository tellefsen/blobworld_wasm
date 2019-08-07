extern crate stdweb;

mod canvas;

use canvas::Canvas;

fn main() {
    stdweb::initialize();

    let mut canvas = Canvas::new("#canvas", 1000, 1000);
    canvas.resize();
    canvas.clear();

    canvas.draw_circle(500.0, 500.0, 500.0, "red");
  
    stdweb::event_loop();
}