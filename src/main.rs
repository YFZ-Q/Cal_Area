trait Geometry {
    fn area(&self);
}

struct Circle {
    r:f32
}

struct Triangle {
    h:f32,
    b:f32
}

struct Square {
    l:f32
}

impl Geometry for Circle {
    fn area(&self) {
        let op = self.r.powf(2.0);
        let res = op * 3.14;
        println!("The circle's area is {}", res);
    }
}

impl Geometry for Triangle {
    fn area(&self) {
        let res = self.h * self.b / 2.0;
        println!("The triangle's area is {}", res);
    }
}

impl Geometry for Square {
    fn area(&self) {
        let res = self.l.powf(2.0);
        println!("The square's area is {}", res);
    }
}

fn cal_area<T:Geometry>(arg: T) {
    arg.area();
}

pub fn main() {
    cal_area(Circle{r: 3.0});
    cal_area(Triangle{h: 6.0, b: 6.0});
    cal_area(Square{l: 10.0});
}