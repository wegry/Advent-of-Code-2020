#[macro_use]
extern crate lazy_static;
mod day1;
mod day2;
mod day3;

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

    run!(day1, day2, day3);
}
