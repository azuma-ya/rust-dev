#[derive(Debug)]
struct Vec2 {
    x:i32,
    y:i32,
}

impl Vec2 {
    fn add(&self, other:&Vec2)->Vec2{
      return  Vec2 {x:self.x+other.x,y:self.y+other.y}
    }
}

fn main() {
    let a = Vec2{x:20,y:10};
    println!("{:?}",a.add(&Vec2{x:20,y:10}));
}
