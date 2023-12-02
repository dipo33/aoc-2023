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
    day_02::solve(true, true);
}

#[allow(dead_code)]
fn test() {
    day_01::test_first(142);
    day_01::test_second(281);

    day_02::test_first(8);
    day_02::test_second(2286);
}


fn main() {
    test();
    solve();
}
