use std::time::{Duration, Instant};

pub fn time_fn<F, I, O>(f: F, input: I) -> (O, Duration)
where
    F: FnOnce(I) -> O,
{
    let start_time = Instant::now();
    let output = f(input);
    (output, start_time.elapsed())
}
