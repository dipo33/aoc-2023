use day_01;
use day_02;


#[allow(dead_code)]
fn measure() {
    let attempts: u32 = 100;
    day_01::measure(attempts);
    day_02::measure(attempts);
}

#[allow(dead_code)]
fn solve() {
    day_01::solve(true, true);
    day_02::solve(false, false);
}

#[allow(dead_code)]
fn test() {
    day_01::test_first(142);
    day_01::test_second(281);
}


fn main() {
    test();
    solve();
}
