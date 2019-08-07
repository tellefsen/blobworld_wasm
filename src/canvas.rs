use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::html_element::CanvasElement;
use stdweb::web::{document, CanvasRenderingContext2d};

pub struct Canvas {
    pub canvas: CanvasElement,
    pub ctx: CanvasRenderingContext2d,
    width: u32,
    height: u32,
}


impl Canvas {
    pub fn new(attr_id: &str, width: u32, height: u32) -> Canvas {
        let canvas: CanvasElement = document()
            .query_selector(attr_id)
            .unwrap()
            .unwrap()
            .try_into()
            .unwrap();

        let ctx: CanvasRenderingContext2d = canvas.get_context().unwrap();

        Canvas {
            canvas,
            ctx,
            width,
            height,
        }
    }

    pub fn resize(&mut self) {
        let window_width = stdweb::web::window().inner_width() as u32;
        let window_height = stdweb::web::window().inner_height() as u32;
        self.canvas.set_width(window_width);
        self.canvas.set_height(window_height);
    }

    pub fn draw_circle(&self, x: f64, y: f64, radius: f64, color: &str) {
        let width_scale = self.canvas.width() as f64 / self.width as f64;
        let height_scale = self.canvas.height() as f64 / self.height as f64;
        self.ctx.set_fill_style_color(color);
        self.ctx.arc(
            x * width_scale,
            y * height_scale,
            radius,
            0.0,
            2.0 * std::f64::consts::PI,
            false
        );
        self.ctx.fill(stdweb::web::FillRule::NonZero);
    }

    pub fn clear(&self) {
        self.ctx.set_fill_style_color("white");
        self.ctx.fill_rect(
            0.0,
            0.0,
            f64::from(self.canvas.width()),
            f64::from(self.canvas.height()),
        );
    }
}