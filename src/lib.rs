#[derive(Debug)]
pub struct Point {
    x: f64,
    y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }

    fn is_in_circle(&self) -> bool {
        self.x * self.x + self.y * self.y <= 1.0
    }
}

fn compute(in_circle: f64, count: f64) -> f64 {
    in_circle as f64 / count as f64 * 4.0
}


pub fn loop_for<S>(count: i32, mut func: S) -> (f64, S) where S: FnMut() -> Point {
    let mut in_circle = 0;
    for _i in 0..count {
        let p = func();
        in_circle += if p.is_in_circle() { 1 } else { 0 }
    }
    let pi = compute(in_circle as f64, count as f64);
    (pi, func)
}


pub fn loop_iter<S>(count: i32, mut func: S) -> (f64, S) where S: FnMut() -> Point {
    let in_circle = (0..count)
        .map(|_i| func())
        .filter(|p| p.is_in_circle())
        .count();
    let pi = compute(in_circle as f64, count as f64);
    (pi, func)
}



