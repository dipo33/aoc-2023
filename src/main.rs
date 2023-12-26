#[allow(dead_code)]
fn measure() {
    let attempts: u32 = 100;
    day_01::measure(attempts);
    day_02::measure(attempts);
    day_03::measure(attempts);
    day_04::measure(attempts);
    day_05::measure(attempts);
    day_06::measure(attempts);
    day_07::measure(attempts);
    day_08::measure(attempts);
}

#[allow(dead_code)]
fn solve() {
    day_01::solve(true, true);
    day_02::solve(true, true);
    day_03::solve(true, true);
    day_04::solve(true, true);
    day_05::solve(true, true);
    day_06::solve(true, true);
    day_07::solve(true, true);
    day_08::solve(true, true);
}

#[allow(dead_code)]
fn test() {
    day_01::test_first(142);
    day_01::test_second(281);

    day_02::test_first(8);
    day_02::test_second(2286);

    day_03::test_first(4361);
    day_03::test_second(467835);

    day_04::test_first(13);
    day_04::test_second(30);

    day_05::test_first(35);
    day_05::test_second(46);

    day_06::test_first(288);
    day_06::test_second(71503);

    day_07::test_first(6440);
    day_07::test_second(5905);

    day_08::test_first(2, 6);
    day_08::test_second(6);
}


fn main() {
    test();
    solve();
    measure();
}
