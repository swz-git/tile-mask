use tile_mask::grid_to_css_path;

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

    println!("{}", grid_to_css_path(grid, 10))
}
