use crate::aod;

fn sorted_append<T>(vector: &mut Vec<T>, new_val: T)
where
    T: Ord + Copy,
{
    match vector
        .iter()
        .enumerate()
        .find(|(_, v)| **v <= new_val)
        .map(|(idx, _)| idx)
    {
        Some(insert_idx) => {
            vector.insert(insert_idx, new_val);
        }
        None => {
            vector.push(new_val);
        }
    }
}

pub fn solution_part_1(input_path: &str) -> aod::SResult<i64> {
    let input = aod::solution_input_to_list_of_strings(input_path)?;

    let mut l1 = Vec::<i64>::new();
    let mut l2 = Vec::<i64>::new();

    for line in input.iter() {
        let two_numbers = line
            .split_whitespace()
            .map(|s| {
                s.parse::<i64>()
                    .map_err(|err| format!("Unable to parse: {s} : {err}"))
            })
            .take(2)
            .collect::<Result<Vec<i64>, _>>()?;

        sorted_append(&mut l1, two_numbers[0]);
        sorted_append(&mut l2, two_numbers[1]);
    }

    Ok(l1
        .into_iter()
        .zip(l2.into_iter())
        .map(|(v1, v2)| (v1 - v2).abs())
        .sum())
}

pub fn solution_part_2(input_path: &str) -> aod::SResult<i64> {
    let input = aod::solution_input_to_list_of_strings(input_path)?;

    let mut l1 = Vec::<i64>::new();
    let mut l2 = Vec::<i64>::new();

    for line in input.iter() {
        let two_numbers = line
            .split_whitespace()
            .map(|s| {
                s.parse::<i64>()
                    .map_err(|err| format!("Unable to parse: {s} : {err}"))
            })
            .take(2)
            .collect::<Result<Vec<i64>, _>>()?;
        l1.push(two_numbers[0]);
        l2.push(two_numbers[1]);
    }

    Ok(l1
        .into_iter()
        .map(|l1_v| l2.iter().filter(|l2_v| **l2_v == l1_v).count() as i64 * l1_v)
        .sum())
}
