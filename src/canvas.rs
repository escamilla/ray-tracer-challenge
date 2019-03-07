use crate::tuples::Tuple;

struct Canvas {
    width: u32,
    height: u32,
    pixels: Vec<Tuple>,
}

impl Canvas {
    pub fn new(width: u32, height: u32) -> Canvas {
        let pixels = vec![Tuple::color(0.0, 0.0, 0.0); (width * height) as usize];
        Canvas {
            width,
            height,
            pixels,
        }
    }

    pub fn write_pixel(&mut self, x: u32, y: u32, color: Tuple) {
        self.pixels[((y * self.width) + x) as usize] = color;
    }

    pub fn pixel_at(&self, x: u32, y: u32) -> Tuple {
        self.pixels[((y * self.width) + x) as usize]
    }
}

#[test]
pub fn test_creating_a_canvas() {
    let c = Canvas::new(10, 20);
    assert_eq!(c.width, 10);
    assert_eq!(c.height, 20);
    for pixel in c.pixels {
        assert_eq!(pixel, Tuple::color(0.0, 0.0, 0.0));
    }
}

#[test]
pub fn test_writing_pixels_to_a_canvas() {
    let mut c = Canvas::new(10, 20);
    let red = Tuple::color(1.0, 0.0, 0.0);
    c.write_pixel(2, 3, red);
    assert_eq!(c.pixel_at(2, 3), red);
}
