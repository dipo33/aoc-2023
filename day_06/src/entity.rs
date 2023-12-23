pub struct Race {
    pub time: u64,
    pub record: u64,
}

impl Race {
    pub fn winning_times_count(&self) -> u32 {
        let (r1, r2) = square_roots(-1, self.time as i64, -(self.record as i64));
        let r1_clamped = (r1 + 1f64).floor() as u32;
        let r2_clamped = (r2 - 1f64).ceil() as u32;

        r2_clamped - r1_clamped + 1
    }
}

fn square_roots(a: i64, b: i64, c: i64) -> (f64, f64) {
    let discriminant = discriminant(a, b, c);
    let squared_discriminant = (discriminant as f64).sqrt();
    let r1 = (-(b as f64) + squared_discriminant) / -2f64;
    let r2 = (-(b as f64) - squared_discriminant) / -2f64;
    (r1, r2)
}

fn discriminant(a: i64, b: i64, c: i64) -> i64 {
    b * b - 4i64 * a * c
}
