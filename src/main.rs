use image::io::Reader;
use image::imageops::grayscale;
use std::io::{stdin, Cursor, Read};


#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn bresenham_to(&self, end : Point) -> Vec<Point> {
        let dx = (end.x - self.x).abs();
        let dy = (end.y - self.y).abs();
        let sx = if self.x < end.x {1} else {-1};
        let sy = if self.y < end.y {1} else {-1};
        let mut err = dx - dy;

        let mut loc = self.clone();
        let mut line = vec![loc.clone()];

        while loc.x != end.x && loc.y != end.y {
            if err * 2 > -dy {
                err -= dy;
                loc.x += sx;
            }
            if err * 2 < dx {
                err += dx;
                loc.y += sy;
            }
            line.push(loc.clone());
        }

        line
    }
}


fn main() -> std::io::Result<()> {
    let mut data = Vec::new();
    stdin().lock().read_to_end(&mut data)?;
    let original = Reader::new(Cursor::new(data)).with_guessed_format()?.decode().expect("Could not decode image.");

    let _working = grayscale(&original);

    let start = Point{x: 5, y: 10};
    for pt in start.bresenham_to(Point{x: 30, y: 40}) {
        println!("x: {}, y: {}", pt.x, pt.y);
    }

    Ok(())
}
