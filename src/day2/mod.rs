use advent_of_code_2025::read_file;

/// rows - reports
///
/// cols - levels
///
/// find safe reports, such that safe is defined as:
/// 1. levels should be strictly monotonic sequence [increasing/decreasing].
/// 2.  1 <= |l0 - l1| < 3 such that l0, l1 are adjacent levels.
pub fn run_1(filename: &str) -> i32{
    let reports = read_file(2, filename).collect::<Vec<_>>();

    let mut safe_reports = 0;

    for i in 0..reports.len(){
        let report = reports[i].as_ref().unwrap();
        let elements: Result<Vec<i32>, std::num::ParseIntError> = report.split_ascii_whitespace().map(|l| l.parse::<i32>()).collect();
        let levels = elements.as_ref().unwrap();

        let mut safe = false;

        // this decides whether monotonous, threshold inbound of {1, 4}
        let monotonous_factor = match(levels[0], levels[1]){
            (prev, curr) if prev > curr => -1, // decreasing
            (prev, curr) if prev < curr =>  1, // increasing
            _ => continue,
        };

        for j in 1..levels.len(){

            let threshold = monotonous_factor * (levels[j] - levels[j - 1]);

            if threshold > 0 && threshold < 4 {
                safe = true;
            }
            else {
                safe = false;
                break;
            }
        }

        if safe {
            safe_reports += 1;
        }
    }

    safe_reports
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_happy_path(){
        assert_eq!(2, run_1("happy_path.txt"));
    }

    #[test]
    fn test_input(){
        assert_eq!(591, run_1("input.txt"));
    }
}