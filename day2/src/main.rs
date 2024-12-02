use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    let safe_lines = contents
        .lines()
        .filter(|line| {
            let numbers: Vec<isize> = line
                .split(" ")
                .map(|x| x.parse::<isize>().expect("Pas de numéro trouvé?"))
                .collect();

            enum LevelsType {
                Desc,
                Asc,
            }

            let level_type = if numbers[0] > numbers[1] {
                LevelsType::Desc
            } else {
                LevelsType::Asc
            };

            for (index, val) in numbers.iter().enumerate() {
                if index == 0 {
                    continue;
                }

                let diff = numbers.get(index - 1).expect("wtf?") - val;

                if matches!(level_type, LevelsType::Desc) {
                    if diff < 1 || diff > 3 {
                        return false;
                    }
                } else if diff > -1 || diff < -3 {
                    return false;
                }
            }

            return true;
        })
        .collect::<Vec<&str>>();

    println!("Part1: Safe lines: {}", safe_lines.len());
}
