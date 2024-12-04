use std::str::Bytes;

use crate::aod;

enum CharacterType {
    Literal(char),
    Number,
    MaybeNumber,
}

type SubPattern = Vec<CharacterType>;
type Pattern = Vec<SubPattern>;

fn expect_subpattern(raw_input: &mut Bytes, subpattern: &SubPattern) -> Option<String> {
    use CharacterType::*;

    let mut result = String::new();
    let mut subpattern_it = subpattern.iter();

    let mut advance_character = true;
    let mut character: Option<char> = None;

    loop {
        if advance_character {
            // Read next byte, without actually modyfing `raw_input`
            character = Some(raw_input.clone().next()? as char);
            advance_character = false;
        }

        assert!(character.is_some());
        debug_assert!(!advance_character);

        match &subpattern_it.next() {
            Some(Literal(l)) => {
                if character.unwrap() != *l {
                    return None;
                }
                advance_character = true;
            }
            Some(Number) => {
                if character.unwrap() >= '0' && character.unwrap() <= '9' {
                    advance_character = true;
                } else {
                    return None;
                }
            }
            Some(MaybeNumber) => {
                if character.unwrap() >= '0' && character.unwrap() <= '9' {
                    advance_character = true;
                } else {
                    advance_character = false;
                }
            }
            None => {
                break;
            }
        }

        if advance_character {
            result.push(character.unwrap());
            raw_input.next();
        }
    }

    Some(result)
}

fn expect_pattern(input: &mut Bytes, pattern: Pattern) -> Option<Vec<String>> {
    let mut input_copy = input.clone();
    let mut subresults = Vec::new();
    for subpattern in pattern.iter() {
        subresults.push(expect_subpattern(&mut input_copy, subpattern)?);
    }
    // If we succeded, modify original input
    *input = input_copy;
    return Some(subresults);
}

fn expect_mul(mut input: &mut Bytes) -> Option<(i64, i64)> {
    use CharacterType::*;
    let pattern = vec![
        vec![Literal('m'), Literal('u'), Literal('l'), Literal('(')],
        vec![Number, MaybeNumber, MaybeNumber],
        vec![Literal(',')],
        vec![Number, MaybeNumber, MaybeNumber],
        vec![Literal(')')],
    ];

    let extracted_substrs = expect_pattern(&mut input, pattern)?;

    let var1 = extracted_substrs[1].parse::<i64>().ok()?;
    let var2 = extracted_substrs[3].parse::<i64>().ok()?;

    Some((var1, var2))
}

pub fn solution_part_1(input_path: &str) -> aod::SResult<i64> {
    let raw_input = aod::solution_input_to_long_string(input_path)?;
    let mut input = raw_input.bytes();

    let mut acc = 0;

    loop {
        match expect_mul(&mut input) {
            Some((a, b)) => {
                acc += a * b;
            }
            None => {
                if input.next().is_none() {
                    break;
                }
            }
        }
    }

    Ok(acc)
}

fn expect_do(mut input: &mut Bytes) -> Option<()> {
    use CharacterType::*;
    let pattern = vec![vec![Literal('d'), Literal('o'), Literal('('), Literal(')')]];
    expect_pattern(&mut input, pattern)?;
    Some(())
}

fn expect_dont(mut input: &mut Bytes) -> Option<()> {
    use CharacterType::*;
    let pattern = vec![vec![
        Literal('d'),
        Literal('o'),
        Literal('n'),
        Literal('\''),
        Literal('t'),
        Literal('('),
        Literal(')'),
    ]];
    expect_pattern(&mut input, pattern)?;
    Some(())
}

pub fn solution_part_2(input_path: &str) -> aod::SResult<i64> {
    let raw_input = aod::solution_input_to_long_string(input_path)?;
    let mut input = raw_input.bytes();

    let mut acc = 0;
    let mut mul_enabled = true;

    loop {
        let got_do = expect_do(&mut input).is_some();
        let got_dont = expect_dont(&mut input).is_some();
        debug_assert!(!(got_do && got_dont));

        if got_do {
            mul_enabled = true;
        }
        if got_dont {
            mul_enabled = false;
        }

        match expect_mul(&mut input) {
            Some((a, b)) => {
                if mul_enabled {
                    acc += a * b;
                }
                continue;
            }
            None => {
                if !(got_do || got_dont) {
                    if input.next().is_none() {
                        break;
                    }
                }
            }
        }
    }

    Ok(acc)
}
