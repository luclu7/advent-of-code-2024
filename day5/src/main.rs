use std::fs;

#[derive(Debug)]
struct PageRule {
    page: usize,
    before: usize,
}

fn main() {
    let text = fs::read_to_string("input.txt").expect("Erreur lecture input.txt");

    let blocks: Vec<_> = text.split("\n\n").collect();

    let definitions = blocks.first().expect("Pas de premier block");
    let pages_to_update_list = blocks.get(1).expect("Pas de second block");

    let pages_rules: Vec<_> = definitions
        .lines()
        .map(|x| {
            let nbs: Vec<_> = x.split("|").collect();

            let get_nb = |i: usize| -> usize {
                nbs.get(i)
                    .expect("Pas de 2nd nombre?")
                    .parse()
                    .expect("erreur parse")
            };

            let page = get_nb(0);
            let before = get_nb(1);

            return PageRule { page, before };
        })
        .collect();

    let pages_to_update: Vec<Vec<usize>> = pages_to_update_list
        .lines()
        .map(|x| {
            x.split(",")
                .map(|page| page.parse::<usize>().expect("Erreur parse"))
                .collect()
        })
        .collect();

    let correctly_ordered_pages: Vec<_> = pages_to_update
        .iter()
        .filter(|pages| {
            // loop over the rules and check one by one
            for rule in pages_rules.iter().clone() {
                let find = |looking_for: usize| pages.iter().position(|page| looking_for == *page);
                if pages.contains(&rule.page) && pages.contains(&rule.before) {
                    if find(rule.page) > find(rule.before) {
                        return false;
                    }
                }
            }

            return true;
        })
        .collect();

    println!("Correctly ordered pages: {:?}", correctly_ordered_pages);

    let main_values = correctly_ordered_pages
        .iter()
        .map(|pages| pages[pages.len() / 2])
        .reduce(|acc, e| acc + e)
        .unwrap();

    println!(
        "Part1: Sum of the middle page numbers of the correct ones: {:?}",
        main_values
    );
}
