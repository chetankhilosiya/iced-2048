pub fn slide_up(values: &mut [[u32; 4]; 4]) {
    println!("before up: {:?}", values);
    for j in 0..4 {
        let mut val = Vec::with_capacity(4);
        for i in 0..4 {
            if values[i][j] != 0 {
                val.push(values[i][j])
            }
        }
        if val.len() < 2 {
            update_column(values, &val, j);
            continue;
        }
        let mut merge = true;
        for i in 1..val.len() {
            if !merge {
                merge = true;
            } else if val[i - 1] == val[i] {
                val[i - 1] += val[i];
                val.remove(i);
                merge = false;
            }
        }
        update_column(values, &val, j);
    }
    println!("after up: {:?}", values);
    todo!("Return updated score");
}

fn update_column(values: &mut [[u32; 4]; 4], val: &[u32], col: usize) {
    for i in 0..4 {
        values[i][col] = if i < val.len() { val[i] } else { 0 }
    }
}
