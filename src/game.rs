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
    for i in 1..val.len() {
        if !merge {
            merge = true;
        } else if val[i - 1] == val[i] {
            val[i - 1] += val[i];
            new_score += u64::from(val[i]);
            val.remove(i);
            merge = false;
        }
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
