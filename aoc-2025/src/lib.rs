pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;

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
            let input = $day::$process(input);
            let elapsed = start.elapsed();
            println!("  {:<15} {:<20}elapsed: {:.3?}", "preprocessing", "", elapsed);
        )?

        $(
            let start = std::time::Instant::now();
            let result = $day::$part(&input);
            let elapsed = start.elapsed();

            println!("  {:<15} {:<20}elapsed: {:.3?}", stringify!($part), result, elapsed);
        )+
        let elapsed = start_day.elapsed();
        println!("elapsed day: {:.3?}", elapsed);
        println!("");
    }};
}
