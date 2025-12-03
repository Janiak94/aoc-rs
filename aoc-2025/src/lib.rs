pub mod day1;
pub mod day2;
pub mod day3;

#[macro_export]
macro_rules! aoc {
    ($day:ident, $($part:ident),+ $(,)?) => {{
        let input = include_str!(concat!("../input/", stringify!($day), ".txt"));

        let start_day = std::time::Instant::now();
        println!("Day {}: ", stringify!($day));
        $(
            let start = std::time::Instant::now();
            let result = aoc_2025::$day::$part(input);
            let elapsed = start.elapsed();

            println!("  {}: {}, elapsed: {:.3?}", stringify!($part), result, elapsed);
        )+
        let elapsed = start_day.elapsed();
        println!("elapsed day: {:.3?}", elapsed);
        println!("");
    }};
}
