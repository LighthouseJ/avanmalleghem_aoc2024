use std::error::Error;

use aoc1::read_file_and_compute_distance;

fn main() -> Result<(), Box::<dyn Error>> {
    let distance = read_file_and_compute_distance("./../input")?;
    println!("The answer is : {}", distance);
    Ok::<_, _>(())
}