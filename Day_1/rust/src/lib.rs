use std::{error::Error, fs::File, io::{self, BufRead, BufReader, Lines}, path::Path};

/// read a file and compute distance
pub fn read_file_and_compute_distance<P: AsRef<Path>>(file_path: P) -> Result<i32, Box<dyn Error>> {
    let (v1, v2) = get_sorted_vectors(file_path)?;
    Ok::<_, _>(compute_distance(&v1, &v2))
}

/// read lines from a file, try and parse each line for 2 i32, then sort and return 2 vecs
fn get_sorted_vectors<P: AsRef<Path>>(file_path: P) -> io::Result<(Vec<i32>, Vec<i32>)> {
    let vec_of_2tuples: Vec<_> = 
        // try and read lines. return fn w/ error if needed
        read_lines(file_path)?
            // consume iterator into FromIterator
            .into_iter()
            // keep going until error, if one. unwraps string inside of Ok until doing so
            .scan((), |_, s| s.ok())
            // parse line. expect parse to succeed reading i32's twice
            .map(|line| {
                let mut s = line.split_whitespace().map(|s| s.parse::<i32>().unwrap());
                (s.next().unwrap(), s.next().unwrap()) // missed opportunity to handle missing values
            })
            .collect();

    // unzip, assign back w/ explicit type and muts, then sort
    let (mut v1, mut v2): (Vec<i32>, Vec<i32>) = vec_of_2tuples.into_iter().unzip();
    v1.sort();
    v2.sort();

    Ok::<_, _>((v1, v2))
}

/// compute distance between 2 columns of vals and sum them up
fn compute_distance(sorted_v1: &[i32], sorted_v2: &[i32]) -> i32 {
    sorted_v1
        .iter()
        .zip(sorted_v2)
        .map(|(c1, c2)| (c1 - c2).abs())
        .sum()
}

/// try and open a file, return Lines iterator
fn read_lines<P>(filename: P) -> io::Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use crate::read_file_and_compute_distance;

    #[test]
    fn test_read_file_and_compute_distance() -> Result<(), Box<dyn Error>>{

        let expected = 1258579; // value determined before changes made

        let my_input = "./../input";

        let actual = read_file_and_compute_distance(my_input)?;
        assert_eq!(expected, actual);
        Ok(())
    }
}