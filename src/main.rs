#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate maplit;
mod day1;
mod day10;
mod day11;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn main() {
    // https://notes.iveselov.info/programming/time_it-a-case-study-in-rust-macros#with-tt-specifier
    macro_rules! timed {
     ($context:literal, $b:block) => {
        println!("{}:", $context);
        let timer = std::time::Instant::now();
        $b
        println!("took {:?}\n", timer.elapsed());
    };
    };

    macro_rules! run {
        ($($module:tt),*) => (
            $(
                println!("Day {}", $module::DAY);
                timed!("Part 1", {$module::part_1()});
                timed!("Part 2", {$module::part_2()});
            )*
        );
    };

    run!(day1, day2, day3, day4, day5, day6, day7, day8, day9, day10, day11);
}
