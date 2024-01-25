use std::num::sqrt;

#[derive(Debug)]
struct Vec2 {
    x: i32,
    y: i32,
}

impl Vec2 {
    fn add(&self, other: &Vec2) -> Vec2 {
        return Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
    fn sub(&self, other: &Vec2) -> Vec2 {
        return Vec2 {
            x: self.x - other.x,
            y: self.y - other.y,
        };
    }
    fn mult(&self, times: i32) -> Vec2 {
        return Vec2 {
            x: self.x * times,
            y: self.y * times,
        };
    }
}

struct Complex {
    a: i32,
    b: i32,
}

impl Complex {
    fn add(&self, other: &Complex) -> Complex {
        Complex {
            a: self.a + other.a,
            b: self.b + other.b,
        }
    }
    fn sub(&self, other: &Complex) -> Complex {
        Complex {
            a: self.a - other.a,
            b: self.b - other.b,
        }
    }
    fn times(&self, other: &Complex) -> Complex {
        Complex {
            a: self.a * other.a - self.b * other.b,
            b: self.a * other.b + self.b * other.a,
        }
    }
    fn div(&self, other: &Complex) -> Complex {
        Complex {
            a: self.a * other.a + self.b * other.b,
            b: self.a * other.b - self.b * other.a,
        }
    }
    fn abs(&self) -> i32 {
        ((self.a.pow(2) + self.b.pow(2)) as f64).sqrt() as i32
    }
}

fn main() {
    let a = Vec2 { x: 20, y: 10 };
    let b = Vec2 { x: 20, y: 10 };
    println!("{:?}", a.sub(&b).add(&b).mult(10));

    let p = Complex { a: 5, b: 10 };
    let r = Complex { a: 3, b: 5 };
    println!("{:?}", p.sub(&r).add(&r).abs());
}
