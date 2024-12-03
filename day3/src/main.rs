use regex::Regex;
use std::fs;
use std::ops::Mul;

fn main() {
    let part1regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    // let part2regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let text = fs::read_to_string("input.txt")
        .expect("erreur lors de la lecture du fichier")
        .replace("\n", ""); // on supprime les retours à la ligne, inutiles

    let total = |regex: Regex, text: &str| -> usize {
        regex
            .captures_iter(text)
            .map(|c| c.extract())
            .fold(0, |acc, (_, [x_str, y_str])| {
                let x: usize = x_str.parse().expect("uh");
                let y: usize = y_str.parse().expect("uh");
                return acc + (x.mul(y));
            })
    };

    println!("Total (part 1): {}", total(part1regex, &text));
    // println!("Total (part 2): {}", total(part2regex, &text));
}
