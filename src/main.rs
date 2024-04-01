fn find_border_sides(grid: Vec<Vec<bool>>) -> Vec<((isize, isize), (isize, isize))> {
    let mut draw_lines = vec![];

    let max_y = (grid.len() - 1) as isize;
    let max_x = (grid[0].len() - 1) as isize;

    // find every side of a tile that is in between a '0' and 'X'
    for (y, row) in grid.iter().enumerate() {
        let y = y as isize;
        for (x, tile) in row.iter().enumerate() {
            let x = x as isize;
            if *tile == false {
                continue;
            }
            let to_check = [
                ('w', x, y - 1), // above
                ('a', x - 1, y), // left
                ('s', x, y + 1), // below
                ('d', x + 1, y), // right
            ];

            let new_lines = to_check
                .iter()
                .filter(|t| {
                    // if out of bounds
                    if t.1 < 0 || t.2 < 0 || t.1 > max_x || t.2 > max_y {
                        return true;
                    }
                    grid[t.2 as usize][t.1 as usize] == false
                })
                .map(|t| match t.0 {
                    'w' => ((x, y), (x + 1, y)),
                    'a' => ((x, y), (x, y + 1)),
                    's' => ((x, y + 1), (x + 1, y + 1)),
                    'd' => ((x + 1, y), (x + 1, y + 1)),
                    _ => unreachable!(),
                });

            draw_lines.extend(new_lines)
        }
    }

    draw_lines
}

fn grouping(mut border_sides: Vec<((isize, isize), (isize, isize))>) -> Vec<Vec<(isize, isize)>> {
    let mut groups: Vec<Vec<(isize, isize)>> = vec![];
    let mut group: Vec<(isize, isize)> = vec![];
    loop {
        if group.last().is_none() {
            let Some(new_pop) = border_sides.pop() else {
                break;
            };
            group.push(new_pop.0);
            group.push(new_pop.1);
        }
        let Some(to_pop_index) = border_sides
            .iter()
            .position(|line| Some(&line.0) == group.last() || Some(&line.1) == group.last())
        else {
            // couldn't find match so its probably done
            groups.push(group);
            group = vec![];
            continue;
        };

        let new_line = border_sides.remove(to_pop_index);
        if Some(&new_line.0) == group.last() {
            group.push(new_line.1)
        } else {
            group.push(new_line.0)
        }
    }
    groups
}

fn main() {
    #[rustfmt::skip]
    let grid: Vec<Vec<bool>> = [
        "00XX00",
        "0XXX00",
        "000000",
        "XX000X",
        "00000X",
    ]
    .into_iter()
    .map(|row| row.chars().map(|x| x == 'X').collect::<Vec<bool>>())
    .collect();

    let border_sides = find_border_sides(grid);

    let grouped_paths = grouping(border_sides);

    // print out css path
    let scale = 10;
    let mut out_str = "".to_owned();
    for path in grouped_paths {
        out_str += "M";
        for point in path {
            out_str += &format!(" {} {}", point.0 * scale, point.1 * scale)
        }
        out_str += "\n";
    }

    println!("{out_str}")
}
