static DIRECTIONS: [(usize, usize); 8] = [
    (!0, !0),
    (0, !0),
    (1, !0),
    (!0, 0),
    (1, 0),
    (!0, 1),
    (0, 1),
    (1, 1),
];

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    if minefield.is_empty() || minefield[0].is_empty() {
        return vec!["".to_string(); minefield.len()];
    }

    let height = minefield.len();
    let width = minefield[0].len();

    (0..height)
        .map(|y| {
            (0..width)
                .map(|x| {
                    if minefield[y].as_bytes()[x] == b'*' {
                        '*'
                    } else {
                        match DIRECTIONS
                            .iter()
                            .map(|&(dx, dy)| (x.wrapping_add(dx), y.wrapping_add(dy)))
                            .filter(|&(x, y)| x < width && y < height)
                            .filter(|&(x, y)| minefield[y].as_bytes()[x] == b'*')
                            .count()
                        {
                            0 => ' ',
                            n => (n as u8 + '0' as u8) as char,
                        }
                    }
                })
                .collect()
        })
        .collect()
}
