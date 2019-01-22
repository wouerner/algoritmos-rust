#[derive(Debug)]
pub struct Pontos {
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
}

impl Pontos {
    pub fn exec(&self) -> f64 {
        //sqrt(pow(($x2 - $x1 ), 2) + pow(($y2 - $y1), 2));
        ((self.x2 - self.x1).powf(2.0) + (self.y2 - self.y1).powf(2.0)).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let p = Pontos { x1: 1.0, y1: 1.0, x2: 4.0, y2: 5.0 };

        assert_eq!(p.exec(),5.0);
    }
}
