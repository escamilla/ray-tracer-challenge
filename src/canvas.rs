use crate::i32_clamp;
use crate::tuple::Tuple;

const PPM_LINE_LENGTH: usize = 70;

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pixels: Vec<Tuple>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        let pixels = vec![Tuple::color(0.0, 0.0, 0.0); width * height];
        Canvas {
            width,
            height,
            pixels,
        }
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: Tuple) {
        self.pixels[y * self.width + x] = color;
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> Tuple {
        self.pixels[y * self.width + x]
    }

    pub fn to_ppm(&self) -> String {
        let mut ppm = String::new();
        ppm.push_str(
            format!("P3\n{} {}\n255\n", self.width, self.height).as_str(),
        );
        let mut values: Vec<i32> = Vec::new();
        for pixel in self.pixels.iter() {
            let scaled_color = *pixel * 255.0;
            let red = i32_clamp(scaled_color.red().round() as i32, 0, 255);
            let green = i32_clamp(scaled_color.green().round() as i32, 0, 255);
            let blue = i32_clamp(scaled_color.blue().round() as i32, 0, 255);
            values.push(red);
            values.push(green);
            values.push(blue);
        }
        let mut line = String::new();
        for i in 0..values.len() {
            let value = format!("{}", values[i]);
            if line.len() + 1 + value.len() >= PPM_LINE_LENGTH {
                line.push('\n');
                ppm.push_str(line.as_str());
                line = String::new();
            }
            if line.len() > 0 {
                line.push(' ');
            }
            line.push_str(value.as_str());
            if (i + 1) % (self.width * 3) == 0 {
                line.push('\n');
                ppm.push_str(line.as_str());
                line = String::new();
            }
        }
        line.push('\n');
        ppm.push_str(line.as_str());
        ppm
    }
}

#[test]
fn test_creating_a_canvas() {
    let c = Canvas::new(10, 20);
    assert_eq!(c.width, 10);
    assert_eq!(c.height, 20);
    for pixel in c.pixels {
        assert_eq!(pixel, Tuple::color(0.0, 0.0, 0.0));
    }
}

#[test]
fn test_writing_pixels_to_a_canvas() {
    let mut c = Canvas::new(10, 20);
    let red = Tuple::color(1.0, 0.0, 0.0);
    c.write_pixel(2, 3, red);
    assert_eq!(c.pixel_at(2, 3), red);
}

#[test]
fn test_constructing_the_ppm_header() {
    let c = Canvas::new(5, 3);
    let ppm = c.to_ppm();
    let mut lines = ppm.lines();
    assert_eq!(Some("P3"), lines.next());
    assert_eq!(Some("5 3"), lines.next());
    assert_eq!(Some("255"), lines.next());
}

#[test]
fn test_constructing_the_ppm_pixel_data() {
    let mut c = Canvas::new(5, 3);
    let c1 = Tuple::color(1.5, 0.0, 0.0);
    let c2 = Tuple::color(0.0, 0.5, 0.0);
    let c3 = Tuple::color(-0.5, 0.0, 1.0);
    c.write_pixel(0, 0, c1);
    c.write_pixel(2, 1, c2);
    c.write_pixel(4, 2, c3);
    let ppm = c.to_ppm();
    let mut lines = ppm.lines().skip(3);
    assert_eq!(Some("255 0 0 0 0 0 0 0 0 0 0 0 0 0 0"), lines.next());
    assert_eq!(Some("0 0 0 0 0 0 0 128 0 0 0 0 0 0 0"), lines.next());
    assert_eq!(Some("0 0 0 0 0 0 0 0 0 0 0 0 0 0 255"), lines.next());
}

#[test]
fn test_splitting_long_lines_in_ppm_files() {
    let mut c = Canvas::new(10, 2);
    for y in 0..c.height {
        for x in 0..c.width {
            c.write_pixel(x, y, Tuple::color(1.0, 0.8, 0.6));
        }
    }
    let ppm = c.to_ppm();
    let mut lines = ppm.lines().skip(3);
    assert_eq!(
        Some("255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204"),
        lines.next()
    );
    assert_eq!(
        Some("153 255 204 153 255 204 153 255 204 153 255 204 153"),
        lines.next()
    );
    assert_eq!(
        Some("255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204"),
        lines.next()
    );
    assert_eq!(
        Some("153 255 204 153 255 204 153 255 204 153 255 204 153"),
        lines.next()
    );
}

#[test]
fn test_ppm_files_are_terminated_by_a_newline_character() {
    let c = Canvas::new(5, 3);
    let ppm = c.to_ppm();
    assert!(ppm.ends_with('\n'));
}
