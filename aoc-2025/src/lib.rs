pub mod day1;

#[macro_export]
macro_rules! aoc {
    ($day:ident, $($part:ident),+ $(,)?) => {{
        let input = include_str!(concat!("../input/", stringify!($day), ".txt"));

        println!("Day {}: ", stringify!($day));
        $(
            println!("  {}: {}", stringify!($part), aoc_2025::$day::$part(input));
        )+
        println!("");
    }};
}
