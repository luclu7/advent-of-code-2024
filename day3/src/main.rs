use regex::Regex;
use std::fs;
use std::ops::Mul;

fn main() {
    let re = Regex::new(r"mul\(([1-9]\d{0,2}),([1-9]\d{0,2})\)").unwrap();

    let text = fs::read_to_string("input.txt")
        .expect("erreur lors de la lecture du fichier")
        .replace("\n", ""); // on supprime les retours à la ligne, inutiles

    let total =
        re.captures_iter(&*text)
            .map(|c| c.extract())
            .fold(0, |acc, (_, [x_str, y_str])| {
                let x: isize = x_str.parse().expect("uh");
                let y: isize = y_str.parse().expect("uh");
                // println!("mul({},{})={}", x, y, x * y);
                return acc + (x.mul(y));
            });

    println!("Total (part 1): {}", total);
}
