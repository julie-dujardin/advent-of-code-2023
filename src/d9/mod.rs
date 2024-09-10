use std::fs;

fn parse_file(file_path: &str) -> Vec<Vec<isize>> {
    let file = fs::read_to_string(file_path).unwrap();
    let file_lines = file.lines();
    let mut series = Vec::new();
    for line in file_lines {
        let mut measurements = Vec::new();
        for measurement in line.split(' ') {
            measurements.push(measurement.parse().unwrap())
        }
        series.push(measurements);
    }
    series
}

fn series_diff(measurements: &[isize]) -> Vec<isize> {
    let mut diffs = Vec::new();

    for i in 0..measurements.len() - 1 {
        diffs.push(measurements[i + 1] - measurements[i])
    }

    diffs
}

fn series_diffs(measurements: &[isize]) -> Vec<Vec<isize>> {
    let mut measurements_diffs = vec![series_diff(measurements)];

    while !measurements_diffs.last().unwrap().iter().all(|&x| x == 0) {
        measurements_diffs.push(series_diff(measurements_diffs.last().unwrap()));
    }
    measurements_diffs
}

pub fn mirage1(file_path: &str) -> usize {
    let series = parse_file(file_path);
    let mut sum = 0;

    for measurements in series {
        let measurements_diffs = series_diffs(&measurements);
        let mut guess_diff = 0;

        let mut diffs_iter = measurements_diffs.iter().rev();
        diffs_iter.next();
        for diffs in diffs_iter {
            guess_diff += diffs.last().unwrap()
        }

        sum += guess_diff + measurements.last().unwrap()
    }

    sum as usize
}

pub fn mirage2000(file_path: &str) -> usize {
    let series = parse_file(file_path);
    let mut sum = 0;

    for measurements in series {
        let measurements_diffs = series_diffs(&measurements);
        let mut guess_diff = 0;

        let mut diffs_iter = measurements_diffs.iter().rev();
        diffs_iter.next();
        for diffs in diffs_iter {
            guess_diff = diffs.first().unwrap() - guess_diff
        }

        sum += measurements.first().unwrap() - guess_diff
    }

    sum as usize
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::load_output::load_results;

    #[test]
    fn test_series_diff() {
        assert_eq!(series_diff(&[0, 3, 6, 9, 12, 15]), vec![3, 3, 3, 3, 3]);
        assert_eq!(series_diff(&[3, 3, 3, 3, 3]), vec![0, 0, 0, 0]);
        assert_eq!(
            series_diff(&[10, 13, 16, 21, 30, 45, 68]),
            vec![3, 3, 5, 9, 15, 23]
        );
    }

    #[test]
    fn p1() {
        let (expected_p1, _) = load_results("d9").unwrap();
        assert_eq!(
            mirage1("test-data/d9/input_test1.txt"),
            expected_p1["input_test1"]
        );
        assert_eq!(mirage1("test-data/d9/input.txt"), expected_p1["input"]);
    }

    #[test]
    fn p2() {
        let (_, expected_p2) = load_results("d9").unwrap();
        assert_eq!(
            mirage2000("test-data/d9/input_test1.txt"),
            expected_p2["input_test1"]
        );
        assert_eq!(mirage2000("test-data/d9/input.txt"), expected_p2["input"]);
    }
}
