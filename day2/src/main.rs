use std::fs;

enum LevelsType {
    Desc,
    Asc,
}

fn check(numbers: &Vec<isize>) -> Result<bool, usize> {
    let level_type = if numbers.first().expect("no first?") > numbers.last().expect("no last?") {
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
                return Err(index);
            }
        } else if diff > -1 || diff < -3 {
            return Err(index);
        }
    }

    Ok(true)
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    let safe_lines_part1 = contents
        .lines()
        .filter(|line: &&str| {
            let numbers: Vec<isize> = line
                .split(" ")
                .map(|x| x.parse::<isize>().expect("Pas de numéro trouvé?"))
                .collect();

            return match check(&numbers) {
                Ok(_) => true,
                Err(_) => false,
            };
        })
        .collect::<Vec<&str>>();

    let safe_lines_part2 = contents
        .lines()
        .filter(|line: &&str| {
            let numbers: Vec<isize> = line
                .split(" ")
                .map(|x| x.parse::<isize>().expect("Pas de numéro trouvé?"))
                .collect();

            return match check(&numbers) {
                Ok(_) => true,
                Err(index) => {
                    // with error dampener, we retry by removing a specific number
                    let mut copy: Vec<isize> = numbers.iter().copied().collect();
                    let mut copy_without_first: Vec<isize> = numbers.iter().copied().collect();
                    println!(
                        "Remove {} (id. {}) from numbers ({:?})",
                        copy[index], index, copy
                    );

                    // c'est horrible
                    copy_without_first.remove(0);
                    copy.remove(index);

                    match check(&copy) {
                        Ok(_) => true,
                        Err(_) => {
                            return match check(&copy_without_first) {
                                Ok(_) => true,
                                Err(_) => false,
                            };
                        }
                    }
                }
            };
        })
        .collect::<Vec<&str>>();

    println!("Part1: Safe lines: {}", safe_lines_part1.len());
    println!("Part2: Safe lines: {}", safe_lines_part2.len());
}
