use std::fs;

type Grid<T> = Vec<Vec<T>>;
type XY = (usize, usize);
type XyDelta = (isize, isize);
fn explore((i, j): XY, grid: &Grid<char>, exploration_grid: &mut Grid<bool>) {
    println!(
        "Exploring possibilities from {},{} (letter {})",
        i, j, grid[i][j]
    );

    println!("{:?}", grid[i]);
    println!("{}", grid[i][j]);

    let neighbours = get_neighboring_cells((i, j), grid);

    let chars_to_find = match grid[i][j] {
        'X' => vec!['M'],
        'M' => vec!['X', 'A'],
        'A' => vec!['M', 'S'],
        'S' => vec!['X'],
        _ => vec![],
    };

    // mark this case as explored
    exploration_grid[i][j] = true;

    let a: Vec<_> = neighbours
        .iter()
        .filter(|(x, y)| {
            // selon la lettre, on cherche un set différent
            // XMAS
            return chars_to_find.contains(&grid[*x][*y]);
        })
        .filter(|x| !exploration_grid[x.0][x.1])
        .collect();

    println!(
        "filtered: {:?} ({:?})",
        a,
        a.iter().map(|(x, y)| grid[*x][*y]).collect::<Vec<char>>()
    );

    a.iter().for_each(|(x, y)| {
        explore((*x, *y), grid, exploration_grid);
    });

    // match grid[i][j] {
    //     'X' => {
    //         }
    //     'M' => {}
    //     'A' => {}
    //     'S' => {}
    //     _ => {}
    // }

    let b: Vec<_> = exploration_grid
        .iter()
        .map(|x| x.iter().map(|x2| return if *x2 { 'O' } else { 'X' }))
        .collect::<_>();

    println!("{:?}", b);
}

fn get_neighboring_cells((i, j): (usize, usize), grid: &Grid<char>) -> Vec<XY> {
    let boxes_to_check: Vec<XyDelta> = vec![
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    boxes_to_check
        .iter()
        .filter_map(|(k1, k2)| {
            let a = k1 + i as isize;
            let b = k2 + j as isize;
            if check_oob((a, b), &grid) {
                return Some((a as usize, b as usize));
            }
            return None;
        })
        .collect()
}

fn check_oob<T>((i, j): XyDelta, grid: &Grid<T>) -> bool {
    i < grid.len() as isize && j < grid[0].len() as isize && i >= 0 && j >= 0
}

fn main() {
    let text = fs::read_to_string("input.txt").expect("lecture d'input.txt");

    let grid = text
        .lines()
        .map(|x| x.chars().collect())
        .collect::<Grid<char>>();

    let mut exploration_grid = vec![vec![false; grid[0].len()]; grid.len()];

    explore((4, 6), &grid, &mut exploration_grid);
    // explore((0, 0), &grid);
}
