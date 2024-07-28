pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let height = minefield.len();
    if height == 0 {
        return vec![];
    }

    let width = minefield[0].len();
    if width == 0 {
        return vec!["".to_string(); height];
    }

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
            match v[i][j] {
                '*' => {
                    board[i][j] = std::i8::MIN;

                    for &(dx, dy) in directions.iter() {
                        let x = j.wrapping_add(dx);
                        let y = i.wrapping_add(dy);

                        if x < width && y < height {
                            board[y][x] += 1;
                        }
                    }
                }
                _ => {}
            }
        }
    }

    let mut ans = vec![];

    for line in board.iter() {
        let l: String = line
            .iter()
            .map(|&c| match c {
                0 => " ".to_string(),
                _ if c < 0 => "*".to_string(),
                _ => c.to_string(),
            })
            .collect();

        ans.push(l);
    }

    ans
}
