use aoc_2022::bubble_sort;
use std::io::read_to_string;

fn main() -> std::io::Result<()> {
    let mut elve_calories: Vec<u32> = vec![0];
    let mut num_elves = 0;

    let file = std::fs::File::open("files/input_day1")?;

    for line in read_to_string(file).unwrap().lines() {
        if line.is_empty() {
            num_elves += 1;
            elve_calories.push(0);
            continue;
        }

        elve_calories[num_elves] += line.parse::<u32>().unwrap();
    }

    bubble_sort(&mut elve_calories);

    println!(
        "The total carried by the top 3 elves is: {}",
        elve_calories[..3].iter().sum::<u32>()
    );
    Ok(())
}
