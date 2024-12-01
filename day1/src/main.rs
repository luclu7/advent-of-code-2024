use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    let count = |line: &str| -> (usize, usize) {
        let numbers = line.split("   ").collect::<Vec<_>>();
        if numbers.len() != 2 {
            panic!("Pas normal ça")
        }

        let parsed_numbers: Vec<usize> = numbers
            .iter()
            .map(|x| x.parse::<usize>().expect("pas un numéro"))
            .collect();

        if parsed_numbers.len() != 2 {
            panic!("Pas normal ça")
        }

        return (parsed_numbers[0], parsed_numbers[1]);
    };

    let mut arrays: (Vec<usize>, Vec<usize>) = (vec![], vec![]);
    contents
        .lines()
        .map(count)
        .collect::<Vec<(usize, usize)>>()
        .iter()
        .for_each(|x1| {
            arrays.0.push(x1.0);
            arrays.1.push(x1.1);
        });

    arrays.0.sort();
    arrays.1.sort();

    if arrays.0.len() != arrays.1.len() {
        panic!("les deux vecs sont pas de la même taille?")
    }

    let abs = |x1: usize, x2: usize| -> usize { x1.abs_diff(x2) };

    let mut total = 0;
    for i in arrays.0.iter().enumerate() {
        let first_number = *i.1;
        let index = i.0;
        let second_number = arrays.1[index];

        let sub = abs(first_number, second_number);
        total += sub;
    }

    println!("Part1: Total: {}", total);

    // Part 2
    let get_score = |number: usize| -> usize {
        arrays
            .1
            .iter()
            .filter(|x3| **x3 == number)
            .collect::<Vec<&usize>>()
            .len()
            * number
    };

    let reducer = |acc: usize, e: usize| -> usize { acc + get_score(e) };

    let result = arrays.0.iter().map(|x| *x).fold(0, reducer);

    println!("Part2: Total: {}", result);
}
