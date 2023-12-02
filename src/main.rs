use task_01;
use task_02;


#[allow(dead_code)]
fn measure() {
    let attempts: u32 = 100;
    task_01::measure(attempts);
    task_02::measure(attempts);
}

#[allow(dead_code)]
fn solve() {
    task_01::solve(true, true);
    task_02::solve(false, false);
}


fn main() {
    solve();
}
