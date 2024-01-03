pub fn slide_up(values: &mut [[u32; 4]; 4], score: u64) -> u64 {
    let mut new_score = score;
    for j in 0..4 {
        let mut val = Vec::with_capacity(4);
        for i in 0..4 {
            if values[i][j] != 0 {
                val.push(values[i][j]);
            }
        }
        if val.len() < 2 {
            update_column_up(values, &val, j);
            continue;
        }
        new_score = merge_values(&mut val, new_score);
        update_column_up(values, &val, j);
    }
    new_score
}

fn update_column_up(values: &mut [[u32; 4]; 4], val: &[u32], col: usize) {
    for i in 0..4 {
        values[i][col] = if i < val.len() { val[i] } else { 0 };
    }
}

fn merge_values(val: &mut Vec<u32>, score: u64) -> u64 {
    let mut merge = true;
    let mut new_score = score;
    let mut i = 1;
    while i < val.len() {
        if !merge {
            merge = true;
        } else if val[i - 1] == val[i] {
            val[i - 1] += val[i];
            new_score += u64::from(val[i]);
            val.remove(i);
            merge = false;
        }
        i += 1;
    }
    new_score
}

pub fn slide_down(values: &mut [[u32; 4]; 4], score: u64) -> u64 {
    let mut new_score = score;
    for j in 0..4 {
        let mut val = Vec::with_capacity(4);
        for i in (0..4).rev() {
            if values[i][j] != 0 {
                val.push(values[i][j]);
            }
        }
        if val.len() < 2 {
            update_column_down(values, &val, j);
            continue;
        }
        new_score = merge_values(&mut val, new_score);
        update_column_down(values, &val, j);
    }
    new_score
}

fn update_column_down(values: &mut [[u32; 4]; 4], val: &[u32], col: usize) {
    for i in (0..4).rev() {
        values[i][col] = if (3 - i) < val.len() { val[3 - i] } else { 0 };
    }
}

pub fn slide_left(values: &mut [[u32; 4]; 4], score: u64) -> u64 {
    let mut new_score = score;
    for i in 0..4 {
        let mut val = Vec::with_capacity(4);
        for j in 0..4 {
            if values[i][j] != 0 {
                val.push(values[i][j]);
            }
        }
        if val.len() < 2 {
            update_row_left(values, &val, i);
            continue;
        }
        new_score = merge_values(&mut val, new_score);
        update_row_left(values, &val, i);
    }
    new_score
}

fn update_row_left(values: &mut [[u32; 4]; 4], val: &[u32], row: usize) {
    for j in 0..4 {
        values[row][j] = if j < val.len() { val[j] } else { 0 };
    }
}

pub fn slide_right(values: &mut [[u32; 4]; 4], score: u64) -> u64 {
    let mut new_score = score;
    for i in 0..4 {
        let mut val = Vec::with_capacity(4);
        for j in (0..4).rev() {
            if values[i][j] != 0 {
                val.push(values[i][j]);
            }
        }
        if val.len() < 2 {
            update_row_right(values, &val, i);
            continue;
        }
        new_score = merge_values(&mut val, new_score);
        update_row_right(values, &val, i);
    }
    new_score
}

fn update_row_right(values: &mut [[u32; 4]; 4], val: &[u32], row: usize) {
    for j in (0..4).rev() {
        values[row][j] = if (3 - j) < val.len() { val[3 - j] } else { 0 };
    }
}

pub fn get_next_random_index(grid_values: &[[u32; 4]; 4]) -> (usize, usize) {
    let mut empty_positions = Vec::new();
    for i in 0..4 {
        for j in 0..4 {
            if grid_values[i][j] == 0 {
                empty_positions.push((4 * i) + j);
            }
        }
    }
    if empty_positions.is_empty() {
        return (usize::MAX, usize::MAX);
    }

    let index = fastrand::usize(..empty_positions.len());
    let i = empty_positions[index] / 4;
    let j = empty_positions[index] % 4;
    (i, j)
}

pub fn pass_or_fail(grid_values: &[[u32; 4]; 4]) -> (bool, bool) {
    for arr in grid_values {
        for val in arr {
            if *val == 2048 {
                // Win game
                return (true, true);
            }
            if *val == 0 {
                // empty box present. So continue game.
                return (false, false);
            }
        }
    }
    // check 2 same values are present in consecutive?
    // if 2 same values are present adjecent to each then game is not over.
    // else game over with Loose.
    let proceed = (false, false);
    for i in 0..4 {
        for j in 0..4 {
            let a = grid_values[i][j];
            if i != 3 {
                let b = grid_values[i + 1][j];
                if a == b {
                    return proceed;
                }
            }
            if j != 3 {
                let b = grid_values[i][j + 1];
                if a == b {
                    return proceed;
                }
            }
            if i != 0 {
                let b = grid_values[i - 1][j];
                if a == b {
                    return proceed;
                }
            }
            if j != 0 {
                let b = grid_values[i][j - 1];
                if a == b {
                    return proceed;
                }
            }
        }
    }
    (true, false)
}
