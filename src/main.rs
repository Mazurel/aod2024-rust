mod aod;
mod day1;
mod day2;
mod day3;
mod day4;

macro_rules! aod_solution {
    ($day:ident, expect: $part1_solution:expr, $part2_solution:expr) => {
        let day_label = stringify!($day);

        let first_solution = $day::solution_part_1(format!("inputs/{}.txt", day_label).as_str())
            .expect(format!("{}, part 1 should not fail", day_label).as_str());
        let second_solution = $day::solution_part_2(format!("inputs/{}.txt", day_label).as_str())
            .expect(format!("{}, part 2 should not fail", day_label).as_str());

        println!(
            "{}, part 1 => {} (expected = {})",
            day_label, first_solution, $part1_solution
        );
        if first_solution != ($part1_solution) {
            println!("> Invalid result !");
        }

        println!(
            "{}, part 2 => {} (expected = {})",
            day_label, second_solution, $part2_solution
        );
        if second_solution != ($part2_solution) {
            println!("> Invalid result !");
        }
    };
}

fn main() {
    aod_solution!(day1, expect: 1646452, 23609874);
    aod_solution!(day2, expect: 510, 553);
    aod_solution!(day3, expect: 175615763, 74361272);
    aod_solution!(day4, expect: 2344, 1815);
}
