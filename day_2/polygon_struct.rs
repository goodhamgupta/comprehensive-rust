use std::ops;

#[derive(Debug, Copy, Clone)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    fn magnitude(&self) -> f64 {
        ((self.x * self.x) as f64 + (self.y * self.y) as f64).sqrt()
    }

    fn dist(&self, p2: Point) -> f64 {
        let xdiff = self.x - p2.x;
        let ydiff = self.y - p2.y;
        ((xdiff * xdiff) as f64 + (ydiff * ydiff) as f64).sqrt()
    }
}

impl ops::Add<Point> for Point {
    type Output = Point;

    fn add(self, p2: Point) -> Point {
        Point::new(self.x + p2.x, self.y + p2.y)
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        return (&self.x == &other.x) & (&self.y == &other.y);
    }
}

pub struct Polygon {
    points: Vec<Point>,
}

impl Polygon {
    fn new() -> Self {
        Polygon { points: vec![] }
    }
    fn add_point(&mut self, new_point: Point) {
        self.points.push(new_point)
    }

    fn iter(&self) -> impl Iterator<Item = &Point> {
        self.points.iter()
    }
    fn left_most_point(&self) -> Option<Point> {
        self.points.iter().min_by_key(|point| point.x).copied()
    }

    fn length(&self) -> f64 {
        if self.points.is_empty() {
            return 0.0;
        }
        let mut result = 0.0;
        let mut last_point = self.points[0];
        for point in &self.points[1..] {
            result += last_point.dist(*point);
            last_point = *point;
        }

        result += last_point.dist(self.points[0]);
        result
    }
}

pub struct Circle {
    origin: Point,
    radius: i32,
}

impl Circle {
    fn new(origin: Point, radius: i32) -> Self {
        Circle { origin, radius }
    }

    fn circumference(&self) -> f64 {
        2.0 * std::f64::consts::PI * (self.radius as f64)
    }
}

pub enum Shape {
    Polygon(Polygon),
    Circle(Circle),
}

impl From<Polygon> for Shape {
    fn from(poly: Polygon) -> Self {
        Shape::Polygon(poly)
    }
}

impl From<Circle> for Shape {
    fn from(circle: Circle) -> Self {
        Shape::Circle(circle)
    }
}

impl Shape {
    fn circumference(&self) -> f64 {
        match self {
            Shape::Polygon(poly) => poly.length(),
            Shape::Circle(circle) => circle.circumference(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn round_two_digits(x: f64) -> f64 {
        (x * 100.0).round() / 100.0
    }

    #[test]
    fn test_point_magnitude() {
        let p1 = Point::new(12, 13);
        assert_eq!(round_two_digits(p1.magnitude()), 17.69);
    }

    #[test]
    fn test_point_dist() {
        let p1 = Point::new(10, 10);
        let p2 = Point::new(14, 13);
        assert_eq!(round_two_digits(p1.dist(p2)), 5.00);
    }

    #[test]
    fn test_point_add() {
        let p1 = Point::new(16, 16);
        let p2 = p1 + Point::new(-4, 3);
        assert_eq!(p2, Point::new(12, 19));
    }

    #[test]
    fn test_polygon_left_most_point() {
        let p1 = Point::new(12, 13);
        let p2 = Point::new(16, 16);

        let mut poly = Polygon::new();
        poly.add_point(p1);
        poly.add_point(p2);

        assert_eq!(poly.left_most_point(), Some(p1));
    }

    #[test]
    fn test_polygon_iter() {
        let p1 = Point::new(12, 13);
        let p2 = Point::new(16, 16);

        let mut poly = Polygon::new();
        poly.add_point(p1);
        poly.add_point(p2);

        let points = poly.iter().cloned().collect::<Vec<_>>();
        assert_eq!(points, vec![Point::new(12, 13), Point::new(16, 16)]);
    }

    #[test]
    fn test_shape_circumferences() {
        let mut poly = Polygon::new();
        poly.add_point(Point::new(12, 13));
        poly.add_point(Point::new(17, 11));
        poly.add_point(Point::new(16, 16));

        let shapes = vec![
            Shape::from(poly),
            Shape::from(Circle::new(Point::new(10, 20), 5)),
        ];

        let circumferences = shapes
            .iter()
            .map(Shape::circumference)
            .map(round_two_digits)
            .collect::<Vec<_>>();

        assert_eq!(circumferences, vec![15.48, 31.42]);
    }
}

#[allow(dead_code)]
fn main() {}
