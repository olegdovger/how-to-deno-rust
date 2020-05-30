use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn compute(rounds: u32) -> u32 {
    let mut count: u32 = 0;
    for _ in 0..rounds {
        for _ in 0..rounds {
            count += 1;
        }
    }
    count
}

#[wasm_bindgen]
pub fn factorial(n: u32) -> u32 {
    (1..=n).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute() {
        assert_eq!(compute(3), 9);
        assert_eq!(compute(0), 0);
        assert_eq!(compute(12), 144);
    }

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(3), 6);
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(12), 479001600);
    }
}
