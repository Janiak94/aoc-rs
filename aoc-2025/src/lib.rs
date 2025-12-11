pub mod day1;
pub mod day10;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

#[macro_export]
macro_rules! aoc {
    // Same preprocessing for each part.
    ( $day:ident => $process:ident => $($part:ident),+ $(,)? ) => {{
        let process = |raw: &str| {
            let start = std::time::Instant::now();
            let input = aoc_2025::$day::$process(raw);
            let elapsed = start.elapsed();
            println!("  preprocessing: elapsed: {:.3?}", elapsed);
            input
        };
        aoc!(@run $day => process => $( $part ),+ );
    }};

    // No preprocessing.
    ( $day:ident => $($part:ident),+ $(,)? ) => {{
        fn process(raw: &str) -> &str {
            raw
        }
        aoc!(@run $day => process => $( $part ),+ );
    }};

    // Helper macro.
    ( @run $day:ident => $process:expr => $($part:ident),+ $(,)? ) => {{
        let start_day = std::time::Instant::now();
        let raw = include_str!(concat!("../input/", stringify!($day), ".txt"));

        println!("Day {}: ", stringify!($day));

        let input = $process(raw);

        $(
            let start = std::time::Instant::now();
            let result = aoc_2025::$day::$part(&input);
            let elapsed = start.elapsed();

            println!("  {:<10} {:<20} elapsed: {:.3?}", stringify!($part), result, elapsed);
        )+
        let elapsed = start_day.elapsed();
        println!("elapsed day: {:.3?}", elapsed);
        println!("");
    }};
}
