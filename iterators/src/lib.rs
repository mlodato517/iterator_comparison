pub fn filter_map_filter_inline(nums: &[u64]) -> Vec<u64> {
    nums.iter()
        .filter(|&&n| n % 3 == 0)
        .map(|&n| n / 3)
        .filter(|&n| n % 3 == 0)
        .collect()
}

pub fn fold_inline(nums: &[u64]) -> Vec<u64> {
    nums.iter().fold(Vec::new(), |mut result, &n| {
        if n % 3 == 0 {
            let third = n / 3;
            if third % 3 == 0 {
                result.push(third);
            }
        }
        result
    })
}

pub fn for_loop_inline(nums: &[u64]) -> Vec<u64> {
    let mut result = Vec::new();
    for n in nums {
        if n % 3 == 0 {
            let third = n / 3;
            if third % 3 == 0 {
                result.push(third);
            }
        }
    }
    result
}

fn divisible_by_3(n: u64) -> bool {
    n % 3 == 0
}
fn divide_by_3(n: u64) -> u64 {
    n / 3
}

pub fn filter_map_filter_callback(nums: &[u64]) -> Vec<u64> {
    nums.iter()
        .filter(|&&n| divisible_by_3(n))
        .map(|&n| divide_by_3(n))
        .filter(|&n| divisible_by_3(n))
        .collect()
}

pub fn fold_callback(nums: &[u64]) -> Vec<u64> {
    nums.iter().fold(Vec::new(), |mut result, &n| {
        if divisible_by_3(n) {
            let byte = divide_by_3(n);
            if divisible_by_3(byte) {
                result.push(byte);
            }
        }
        result
    })
}

pub fn for_loop_callback(nums: &[u64]) -> Vec<u64> {
    let mut result = Vec::new();
    for &n in nums {
        if divisible_by_3(n) {
            let third = divide_by_3(n);
            if divisible_by_3(third) {
                result.push(third);
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test {
        ($f:expr) => {
            assert_eq!($f(&[0, 3, 6, 9]), vec![0, 3]);
        };
    }

    #[test]
    fn filter_map_filter_callback_works() {
        test!(filter_map_filter_callback);
    }

    #[test]
    fn fold_callback_works() {
        test!(fold_callback)
    }

    #[test]
    fn single_loop_works() {
        test!(for_loop_callback);
    }

    #[test]
    fn multiple_inline_works() {
        test!(filter_map_filter_inline);
    }

    #[test]
    fn single_inline_works() {
        test!(fold_inline);
    }

    #[test]
    fn single_loop_inline_works() {
        test!(for_loop_inline);
    }
}
