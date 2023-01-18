use std::io::read_to_string;

fn main() -> std::io::Result<()> {
    let mut elve_calories: Vec<u32> = vec![0];
    let mut most_calories = 0;
    let mut num_elves = 0;

    let file = std::fs::File::open("files/input_day1")?;

    for line in read_to_string(file).unwrap().lines() {
        if line.is_empty() {
            if elve_calories[num_elves] > most_calories {
                most_calories = elve_calories[num_elves]
            }
            num_elves += 1;
            elve_calories.push(0);
            continue;
        }

        elve_calories[num_elves] += line.parse::<u32>().unwrap();
    }
    println!("The largest total is: {}", most_calories);
    Ok(())
}
