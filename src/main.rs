mod aod;
mod day1;
mod day2;
mod day3;

fn main() {
    println!(
        "DAY 1 PART 1: {}",
        day1::solution_part_1("inputs/day1.txt").expect("DAY 1 PART 1 should not fail")
    );
    println!(
        "DAY 1 PART 2: {}",
        day1::solution_part_2("inputs/day1.txt").expect("DAY 1 PART 2 should not fail")
    );
    println!(
        "DAY 2 PART 1: {}",
        day2::solution_part_1("inputs/day2.txt").expect("DAY 2 PART 1 should not fail")
    );
    println!(
        "DAY 2 PART 1: {}",
        day2::solution_part_2("inputs/day2.txt").expect("DAY 2 PART 2 should not fail")
    );
    println!(
        "DAY 3 PART 1: {}",
        day3::solution_part_1("inputs/day3.txt").expect("DAY 3 PART 1 should not fail")
    );
    println!(
        "DAY 3 PART 2: {}",
        day3::solution_part_2("inputs/day3.txt").expect("DAY 3 PART 2 should not fail")
    );
}
