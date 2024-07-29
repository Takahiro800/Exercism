pub fn annotate(minefield: &[&str]) -> Vec<String> {
    if minefield.is_empty() || minefield[0].is_empty() {
        return vec!["".to_string(); minefield.len()];
    }

    let height = minefield.len();
    let width = minefield[0].len();
    let v: Vec<Vec<char>> = minefield.into_iter().map(|s| s.chars().collect()).collect();

    let directions = [
        (!0, !0),
        (0, !0),
        (1, !0),
        (!0, 0),
        (1, 0),
        (!0, 1),
        (0, 1),
        (1, 1),
    ];

    let mut board = vec![vec![0_i8; width]; height];

    for i in 0..height {
        for j in 0..width {
            if v[i][j] == '*' {
                board[i][j] = std::i8::MIN;

                for &(dx, dy) in directions.iter() {
                    let x = j.wrapping_add(dx);
                    let y = i.wrapping_add(dy);

                    if x < width && y < height {
                        board[y][x] += 1;
                    }
                }
            }
        }
    }

    board
        .iter()
        .map(|line| {
            line.iter()
                .map(|&c| match c {
                    0 => " ".to_string(),
                    _ if c < 0 => "*".to_string(),
                    _ => c.to_string(),
                })
                .collect()
        })
        .collect()
}
