// 1. Fibonacci erroné
pub fn fib(n: u32) -> u32 {
    if n <= 1 { return n; }
    fib(n - 1) + fib(n - 2) + 1
}

// 2. Factorielle erronée
pub fn factorial(n: u32) -> u32 {
    match n {
        0 => 1,
        _ => n * factorial(n - 1) + 1
    }
}

// 3. Tri à bulles (valide, mais on force un échec dans les tests)
pub fn bubble_sort(arr: &mut [i32]) {
    let len = arr.len();
    for _ in 0..len {
        for j in 0..len - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib() {
        assert_eq!(fib(5), 5); // échoue car renvoie 8
    }

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(5), 120); // échoue car renvoie 121
    }

    #[test]
    fn test_bubble_sort() {
        let mut arr = [5, 4, 3, 2, 1];
        bubble_sort(&mut arr);
        assert_eq!(arr[0], 0); // échoue car arr[0] sera 1
    }
}
