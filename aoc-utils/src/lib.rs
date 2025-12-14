mod input;

pub use input::{download_input_file, read_input_file};

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
        __aoc_impl!($($tree)*);
        let elapsed = start.elapsed();
        println!("Total elapsed: {:.3?}", elapsed);
    }};
}

#[macro_export]
#[doc(hidden)]
macro_rules! run_day {
    ( @run $year:literal $day:ident $( @process $process:ident )? @parts $($part:ident),+ $(,)?) => {{
        let day_usize = stringify!($day).strip_prefix("day").unwrap_or(stringify!($day)).parse::<usize>().expect("Could not parse day ident");
        download_input_file($year, day_usize).expect("Could not download input file");
        let input = &read_input_file($year, day_usize).expect("Could not read input file");

        let start_day = std::time::Instant::now();

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
macro_rules! __aoc_impl {
    ($year:literal $day:ident => $process:ident => $($part:ident),+ $(,)? ;  $($rest:tt)*) => {{
        run_day!(@run $year $day @process $process @parts $($part, )+);
        __aoc_impl!($($rest)*);
    }};

    ($year:literal $day:ident => $($part:ident),+ $(,)? ;  $($rest:tt)*) => {{
        run_day!(@run $year $day @parts $($part, )+);
        __aoc_impl!($($rest)*);
    }};

    () => {{}};
}
