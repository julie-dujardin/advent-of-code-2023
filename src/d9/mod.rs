use std::collections::HashMap;
use std::fs;

fn parse_file(file_path: &str) -> Vec<Vec<isize>> {
    let file = fs::read_to_string(file_path).unwrap();
    let mut file_lines = file.lines();
    let mut series = Vec::new();
    for line in  file_lines{
        let mut measurements = Vec::new();
        for measurement in line.split(' ') {
            measurements.push(measurement.parse().unwrap())
        }
        series.push(measurements);
    }
    series
}

fn series_diff(measurements: &Vec<isize>) -> Vec<isize> {
    let mut diffs = Vec::new();

    for i in 0..measurements.len()-1 {
        diffs.push(measurements[i+1] - measurements[i])
    }

    diffs
}

fn mirage1(file_path: &str) -> isize {
    let series = parse_file(file_path);
    let mut sum = 0;

    for measurements in series {
        let mut measurements_diffs = vec![series_diff(&measurements)];

        while !measurements_diffs.last().unwrap().iter().all(|&x| x == 0) {
            measurements_diffs.push(series_diff(measurements_diffs.last().unwrap()));
        }

        let mut guess_diff = 0;

        let mut diffs_iter = measurements_diffs.iter().rev();
        diffs_iter.next();
        for diffs in diffs_iter {
            guess_diff += diffs.last().unwrap()
        }

        sum += guess_diff + measurements.last().unwrap()
    }
    
    sum
}

fn mirage2000(file_path: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_series_diff() {
        assert_eq!(series_diff(&vec![0, 3, 6, 9, 12, 15]), vec![3, 3, 3, 3, 3]);
        assert_eq!(series_diff(&vec![3, 3, 3, 3, 3]), vec![0, 0, 0, 0]);
        assert_eq!(series_diff(&vec![10, 13, 16, 21, 30, 45, 68]), vec![3, 3, 5, 9, 15, 23]);
    }

    #[test]
    fn p1() {
        assert_eq!(mirage1("src/d9/input_test1.txt"), 114); // provided test
        assert_eq!(mirage1("src/d9/input.txt"), 1696140818);
    }

    #[test]
    fn p2() {
        assert_eq!(mirage2000("src/d9/input_test3.txt"), 6); // provided test
        assert_eq!(mirage2000("src/d9/input.txt"), 13289612809129);
    }
}
