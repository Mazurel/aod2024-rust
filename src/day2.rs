use crate::aod;

type Level = i64;
type Report = Vec<Level>;

#[derive(Debug, Clone, Copy)]
enum Order {
    Increasing,
    Decreasing,
    Unknown,
}

fn process_report_level_pair(
    level_on_left: Level,
    level_on_right: Level,
    current_order: Order,
) -> aod::SResult<Order> {
    use Order::*;
    match current_order {
        Unknown => {
            if level_on_right > level_on_left {
                if (level_on_right - level_on_left) > 3 {
                    Err(format!("Level increased too much !"))
                } else {
                    Ok(Increasing)
                }
            } else if level_on_right < level_on_left {
                if (level_on_left - level_on_right) > 3 {
                    Err(format!("Level decreased too much !"))
                } else {
                    Ok(Decreasing)
                }
            } else {
                Err(format!("Level cannot stay the same"))
            }
        }
        Increasing => {
            if level_on_right <= level_on_left {
                Err(format!("Level decreased or is equal, but should increase"))
            } else if (level_on_right - level_on_left) > 3 {
                Err(format!("Level increased too much !"))
            } else {
                Ok(Increasing)
            }
        }
        Decreasing => {
            if level_on_right >= level_on_left {
                Err(format!("Level increased or is equal, but should decrease"))
            } else if (level_on_left - level_on_right) > 3 {
                Err(format!("Level decreased too much !"))
            } else {
                Ok(Decreasing)
            }
        }
    }
}

fn is_report_valid(report: &Report) -> bool {
    use Order::*;

    let mut order = Unknown;
    let mut previous_level_wrapped = None;
    for (i, level) in report.iter().enumerate() {
        match previous_level_wrapped {
            None => {}
            Some(previous_level) => {
                debug_assert!(i >= 1);
                match process_report_level_pair(previous_level, *level, order) {
                    Ok(new_order) => {
                        order = new_order;
                    }
                    Err(_) => {
                        return false;
                    }
                }
            }
        }
        previous_level_wrapped = Some(*level);
    }

    true
}

fn report_without_level(report: &Report, level_idx: usize) -> Report {
    let mut report_cloned = report.clone();
    report_cloned.remove(level_idx);
    report_cloned
}

fn generic_solution(input_path: &str, tolerate_mistake: bool) -> aod::SResult<i64> {
    let lines_of_reports = aod::solution_input_to_list_of_strings(input_path)?;

    let mut safe = 0;
    'next_report: for raw_report in lines_of_reports.iter() {
        let report: Vec<i64> = raw_report
            .split_whitespace()
            .map(|str| str.parse::<i64>())
            .collect::<Result<Vec<i64>, _>>()
            .map_err(|err| format!("{err}"))?;

        if is_report_valid(&report) {
            safe += 1;
            continue 'next_report;
        }

        if !tolerate_mistake {
            continue 'next_report;
        }

        for i in 0..report.len() {
            let report_without_i = report_without_level(&report, i);
            if is_report_valid(&report_without_i) {
                safe += 1;
                continue 'next_report;
            }
        }
    }

    Ok(safe)
}

pub fn solution_part_1(input_path: &str) -> aod::SResult<i64> {
    generic_solution(input_path, false)
}

pub fn solution_part_2(input_path: &str) -> aod::SResult<i64> {
    generic_solution(input_path, true)
}
