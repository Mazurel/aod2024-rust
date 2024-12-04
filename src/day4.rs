use crate::aod;

struct Board {
    board: Vec<Vec<char>>,
    width: i64,
    height: i64,
}

enum Direction {
    Left,
    Up,
    Right,
    Down,
    Diagonal1,
    Diagonal2,
    Diagonal3,
    Diagonal4,
}

impl Board {
    fn new(input: Vec<String>) -> Self {
        let board: Vec<Vec<char>> = input
            .iter()
            .map(|line| line.chars().into_iter().collect())
            .collect();
        let width = board[0].len() as i64;
        let height = board.len() as i64;
        Board {
            board,
            width,
            height,
        }
    }

    fn at(&self, y: i64, x: i64) -> Option<char> {
        if x < 0 || x >= self.width || y >= self.height || y < 0 {
            None
        } else {
            Some(self.board[y as usize][x as usize])
        }
    }

    fn word_in_direction(&self, y: i64, x: i64, dir: Direction) -> Option<String> {
        const WORD_LEN: usize = 4;

        let mut tmp_y = y as i64;
        let mut tmp_x = x as i64;

        use Direction::*;
        let (dy, dx) = match dir {
            Left => (0, -1),
            Up => (-1, 0),
            Right => (1, 0),
            Down => (0, 1),
            Diagonal1 => (-1, -1),
            Diagonal2 => (-1, 1),
            Diagonal3 => (1, 1),
            Diagonal4 => (1, -1),
        };

        let mut result: String = String::new();
        while let Some(chr) = self.at(tmp_y as i64, tmp_x as i64) {
            result.push(chr);
            tmp_y += dy;
            tmp_x += dx;

            if result.len() >= WORD_LEN {
                break;
            }
        }

        if result.len() == WORD_LEN {
            Some(result)
        } else {
            None
        }
    }

    fn all_possible_words_at(&self, y: i64, x: i64) -> Vec<String> {
        use Direction::*;
        let mut words = Vec::new();
        for direction in [
            Left, Up, Right, Down, Diagonal1, Diagonal2, Diagonal3, Diagonal4,
        ] {
            if let Some(word) = self.word_in_direction(y, x, direction) {
                words.push(word);
            }
        }
        words
    }

    fn is_xmas(&self, y: i64, x: i64) -> Option<()> {
        let center = self.at(y, x)?;
        if center != 'A' {
            return None;
        }

        let top_left = self.at(y - 1, x - 1)?;
        let bottom_right = self.at(y + 1, x + 1)?;
        let bottom_left = self.at(y + 1, x - 1)?;
        let top_right = self.at(y - 1, x + 1)?;

        if top_left == 'M' {
            if bottom_right != 'S' {
                return None;
            }
        } else if top_left == 'S' {
            if bottom_right != 'M' {
                return None;
            }
        } else {
            return None;
        }

        if top_left == top_right {
            if bottom_left != bottom_right {
                return None;
            }
        } else if top_left == bottom_left {
            if bottom_right != top_right {
                return None;
            }
        } else {
            return None;
        }

        Some(())
    }
}

pub fn solution_part_1(input_path: &str) -> aod::SResult<i64> {
    let input = aod::solution_input_to_list_of_strings(input_path)?;
    let board = Board::new(input);

    let mut acc = 0;
    for y in 0..board.height {
        for x in 0..board.width {
            for word in board.all_possible_words_at(y, x) {
                if word == "XMAS" {
                    acc += 1;
                }
            }
        }
    }

    Ok(acc)
}

pub fn solution_part_2(input_path: &str) -> aod::SResult<i64> {
    let input = aod::solution_input_to_list_of_strings(input_path)?;
    let board = Board::new(input);

    let mut acc = 0;
    for y in 0..board.height {
        for x in 0..board.width {
            if board.is_xmas(y, x).is_some() {
                acc += 1;
            }
        }
    }

    Ok(acc)
}
