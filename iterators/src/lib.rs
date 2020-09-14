pub fn filter_map_filter_inline(nums: &[u64]) -> Vec<u64> {
    nums.iter()
        .filter(|&&n| n % 3 == 0)
        .map(|&n| n & (255 << 8))
        .filter(|&n| n % 3 == 0)
        .collect()
}

pub fn fold_inline(nums: &[u64]) -> Vec<u64> {
    nums.iter().fold(Vec::new(), |mut result, &n| {
        if n % 3 == 0 {
            let high_bits = n & (255 << 8);
            if high_bits % 3 == 0 {
                result.push(high_bits);
            }
        }
        result
    })
}

struct Foo<I: Iterator<Item = u64>> {
    inner: I,
}
impl<I: Iterator<Item = u64>> Foo<I> {
    fn fold_custom<F>(mut self, init: Vec<u64>, mut f: F) -> Vec<u64>
    where
        F: FnMut(&mut Vec<u64>, u64),
    {
        let mut accum = init;
        while let Some(x) = self.inner.next() {
            f(&mut accum, x);
        }
        accum
    }
}

pub fn fold_custom(nums: &[u64]) -> Vec<u64> {
    let foo = Foo {
        inner: nums.iter().copied(),
    };
    foo.fold_custom(Vec::new(), |result, n| {
        if n % 3 == 0 {
            let high_bits = n & (255 << 8);
            if high_bits % 3 == 0 {
                result.push(high_bits);
            }
        }
    })
}

pub fn for_loop_inline(nums: &[u64]) -> Vec<u64> {
    let mut result = Vec::new();
    for n in nums {
        if n % 3 == 0 {
            let high_bits = n & (255 << 8);
            if high_bits % 3 == 0 {
                result.push(high_bits);
            }
        }
    }
    result
}

fn divisible_by_3(n: u64) -> bool {
    n % 3 == 0
}
fn second_byte(n: u64) -> u64 {
    n & (255 << 8)
}

pub fn filter_map_filter_callback(nums: &[u64]) -> Vec<u64> {
    nums.iter()
        .filter(|&&n| divisible_by_3(n))
        .map(|&n| second_byte(n))
        .filter(|&n| divisible_by_3(n))
        .collect()
}

pub fn fold_callback(nums: &[u64]) -> Vec<u64> {
    nums.iter().fold(Vec::new(), |mut result, &n| {
        if divisible_by_3(n) {
            let high_bits = second_byte(n);
            if divisible_by_3(high_bits) {
                result.push(high_bits);
            }
        }
        result
    })
}

pub fn for_loop_callback(nums: &[u64]) -> Vec<u64> {
    let mut result = Vec::new();
    for &n in nums {
        if divisible_by_3(n) {
            let high_bits = second_byte(n);
            if divisible_by_3(high_bits) {
                result.push(high_bits);
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter_map_filter_callback_works() {
        assert_eq!(
            filter_map_filter_callback(&[
                0,
                (3 << 8) | 3,
                (4 << 8) | 3,
                (3 << 8) | 4,
                (6 << 8) | 3
            ]),
            vec![0, 3 << 8, 6 << 8]
        )
    }

    #[test]
    fn fold_callback_works() {
        assert_eq!(
            fold_callback(&[0, (3 << 8) | 3, (4 << 8) + 2, (3 << 8) + 1, (6 << 8) | 3]),
            vec![0, 3 << 8, 6 << 8]
        )
    }

    #[test]
    fn single_loop_works() {
        assert_eq!(
            for_loop_callback(&[0, (3 << 8) | 3, (4 << 8) + 2, (3 << 8) + 1, (6 << 8) | 3]),
            vec![0, 3 << 8, 6 << 8]
        )
    }

    #[test]
    fn multiple_inline_works() {
        assert_eq!(
            filter_map_filter_inline(&[0, (3 << 8) | 3, (4 << 8) + 2, (3 << 8) + 1, (6 << 8) | 3]),
            vec![0, 3 << 8, 6 << 8]
        )
    }

    #[test]
    fn single_inline_works() {
        assert_eq!(
            fold_inline(&[0, (3 << 8) | 3, (4 << 8) + 2, (3 << 8) + 1, (6 << 8) | 3]),
            vec![0, 3 << 8, 6 << 8]
        )
    }

    #[test]
    fn single_loop_inline_works() {
        assert_eq!(
            for_loop_inline(&[0, (3 << 8) | 3, (4 << 8) + 2, (3 << 8) + 1, (6 << 8) | 3]),
            vec![0, 3 << 8, 6 << 8]
        )
    }

    #[test]
    fn fold_custom_works() {
        assert_eq!(
            fold_custom(&[0, (3 << 8) | 3, (4 << 8) + 2, (3 << 8) + 1, (6 << 8) | 3]),
            vec![0, 3 << 8, 6 << 8]
        )
    }
}
