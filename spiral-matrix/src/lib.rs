pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let mut matrix = vec![vec![0; size as usize]; size as usize];

    let mut h_left = size as usize;
    let mut v_left = size as usize;
    let mut direction = (1,1);
    let mut current = (-1,0);
    let mut counter = 1;

    while h_left > 0 || v_left > 0 {
        // horizontal movement
        for _ in 0..h_left {
            current.0 += direction.0;
            matrix[current.1 as usize][current.0 as usize] = counter;
            counter += 1;
        }
        v_left -= 1;
        direction.0 *= -1;
        // vertical movement
        for _ in 0..v_left {
            current.1 += direction.1;
            matrix[current.1 as usize][current.0 as usize] = counter;
            counter += 1;
        }

        h_left -= 1;
        direction.1 *= -1;
    }

    matrix
}
