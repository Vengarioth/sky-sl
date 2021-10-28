#[allow(dead_code)]
pub fn measure<F: FnOnce() -> R, R>(f: F) -> R {
    let start = std::time::Instant::now();
    let r = f();
    dbg!(start.elapsed());
    r
}
