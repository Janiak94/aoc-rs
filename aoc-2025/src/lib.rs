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

/// Runs and times each day for the year.
/// Each day is timed and each of its steps, including preprocessing.
///
/// Preprocessing can optionally be provided before each part.
///
/// Usage example:
/// ```
/// mod dayXX;
/// mod dayYY;
/// mod dayXY;
/// mod dayYX;
///
/// aoc!(
///     day01 => part1;
///     dayYY => part1, part2;
///     dayXY => process => part1;
///     dayYX => process => part1, part2;
/// );
/// ```
#[macro_export]
macro_rules! aoc {
    ($($tree:tt)*) => {{
        let start = std::time::Instant::now();
        aoc_impl!($($tree)*);
        let elapsed = start.elapsed();
        println!("Total elapsed: {:.3?}", elapsed);
    }};
}

#[macro_export]
#[doc(hidden)]
macro_rules! run_day {
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

#[macro_export]
#[doc(hidden)]
macro_rules! aoc_impl {
    ($day:ident => $process:ident => $($part:ident),+ $(,)? ;  $($rest:tt)*) => {{
        run_day!(@run $day @process $process @parts $($part, )+);
        aoc_impl!($($rest)*);
    }};

    ($day:ident => $($part:ident),+ $(,)? ;  $($rest:tt)*) => {{
        run_day!(@run $day @parts $($part, )+);
        aoc_impl!($($rest)*);
    }};

    () => {{}};
}
