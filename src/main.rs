#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate maplit;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

fn main() {
    macro_rules! run {
        ($($module:tt),*) => (
            $(
                println!("Day {}", $module::DAY);
                println!("Part 1:");
                $module::part_1();
                println!("Part 2:");
                $module::part_2();
            )*
        );
    };

    run!(day1, day2, day3, day4, day5);
}
