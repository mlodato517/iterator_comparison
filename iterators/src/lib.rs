pub fn multiple_filters_inline(nums: &[u64]) -> Vec<u64> {
    nums.iter()
        .filter(|&&n| n % 3 == 0)
        .filter(|&&n| n % 5 == 0)
        .filter(|&&n| n % 7 == 0)
        .filter(|&&n| n % 11 == 0)
        .copied()
        .collect()
}

pub fn single_filter_inline(nums: &[u64]) -> Vec<u64> {
    nums.iter()
        .filter(|&&n| n % 3 == 0 && n % 5 == 0 && n % 7 == 0 && n % 11 == 0)
        .copied()
        .collect()
}

pub fn single_loop_filter_inline(nums: &[u64]) -> Vec<u64> {
    let mut return_value = Vec::new();
    for n in nums {
        if n % 3 == 0 && n % 5 == 0 && n % 7 == 0 && n % 11 == 0 {
            return_value.push(*n);
        }
    }
    return_value
}

fn divisible_by_3(n: u64) -> bool {
    n % 3 == 0
}
fn divisible_by_5(n: u64) -> bool {
    n % 5 == 0
}
fn divisible_by_7(n: u64) -> bool {
    n % 7 == 0
}
fn divisible_by_11(n: u64) -> bool {
    n % 11 == 0
}
fn divisible_by_1155(n: u64) -> bool {
    divisible_by_3(n) && divisible_by_5(n) && divisible_by_7(n) && divisible_by_11(n)
}

pub fn multiple_filters(nums: &[u64]) -> Vec<u64> {
    nums.iter()
        .filter(|&&n| divisible_by_3(n))
        .filter(|&&n| divisible_by_5(n))
        .filter(|&&n| divisible_by_7(n))
        .filter(|&&n| divisible_by_11(n))
        .copied()
        .collect()
}

pub fn single_filter(nums: &[u64]) -> Vec<u64> {
    nums.iter()
        .filter(|&&n| divisible_by_1155(n))
        .copied()
        .collect()
}

pub fn single_loop_filter(nums: &[u64]) -> Vec<u64> {
    let mut return_value = Vec::new();
    for &n in nums {
        if divisible_by_1155(n) {
            return_value.push(n);
        }
    }
    return_value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiple_works() {
        assert_eq!(multiple_filters(&[1155, 1]), vec![1155])
    }

    #[test]
    fn single_works() {
        assert_eq!(single_filter(&[1155, 1]), vec![1155])
    }

    #[test]
    fn single_loop_works() {
        assert_eq!(single_loop_filter(&[1155, 1]), vec![1155])
    }

    #[test]
    fn multiple_inline_works() {
        assert_eq!(multiple_filters_inline(&[1155, 1]), vec![1155])
    }

    #[test]
    fn single_inline_works() {
        assert_eq!(single_filter_inline(&[1155, 1]), vec![1155])
    }

    #[test]
    fn single_loop_inline_works() {
        assert_eq!(single_loop_filter_inline(&[1155, 1]), vec![1155])
    }

    #[test]
    fn all_equal() {
        let nums: Vec<u64> = (0..100_000).collect();
        let multiple = multiple_filters(&nums);
        let single = single_filter(&nums);
        let single_loop = single_loop_filter(&nums);
        let multiple_inline = multiple_filters_inline(&nums);
        let single_inline = single_filter_inline(&nums);
        let single_loop_inline = single_loop_filter_inline(&nums);

        assert_eq!(multiple, single);
        assert_eq!(multiple, single_loop);
        assert_eq!(multiple, multiple_inline);
        assert_eq!(multiple, single_inline);
        assert_eq!(multiple, single_loop_inline);
    }
}
