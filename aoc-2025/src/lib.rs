pub mod day1;
pub mod day10;
pub mod day11;
pub mod day12;
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
    ( $day:ident => $($part:ident),+ ) => {{
        aoc!(@run $day @parts $( $part, )+);
    }};

    ( $day:ident => $process:ident => $($part:ident),+ ) => {{
        aoc!(@run $day @process $process @parts $( $part, )+);
    }};

    ( @run $day:ident $( @process $process:ident )? @parts $($part:ident),+ $(,)?) => {{
        let start_day = std::time::Instant::now();
        let input = include_str!(concat!("../input/", stringify!($day), ".txt"));

        println!("Day {}: ", stringify!($day));

        $(
            let start = std::time::Instant::now();
            let input = aoc_2025::$day::$process(input);
            let elapsed = start.elapsed();
            println!("  {:<15} {:<20}elapsed: {:.3?}", "preprocessing", "", elapsed);
        )?

        $(
            let start = std::time::Instant::now();
            let result = aoc_2025::$day::$part(&input);
            let elapsed = start.elapsed();

            println!("  {:<15} {:<20}elapsed: {:.3?}", stringify!($part), result, elapsed);
        )+
        let elapsed = start_day.elapsed();
        println!("elapsed day: {:.3?}", elapsed);
        println!("");
    }};
}
