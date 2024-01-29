#![allow(unused)]

pub fn is_prime(number: usize) -> bool {
    let limit = (number as f64).sqrt() as usize;

    if number == 1 {
        return false;
    }

    for i in 2..=limit {
        if number % i == 0 {
            return false;
        }
    }

    return true;
}

pub fn prime_factorization(number: usize) -> Vec<(usize, usize)> {
    let mut num = number;
    let mut result: Vec<(usize, usize)> = vec![];

    let mut i = 2;
    let mut len = 0;

    while i * i <= num {

        if num % i == 0 {
            result.push((i, 0));
            len = result.len() - 1;

            while num % i == 0 {
                num /= i;
                result[len].1 += 1;
            }
        }

        i += 1;
    }
    
    if num > 1 {
        result.push((num,1));
    }
    
    result
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prime_numbers() {
        let numbers: Vec<usize> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
        let answers: Vec<bool> = vec![
            false, true, true, false, true, false, true, false, false, true,
        ];

        for index in 0..answers.len() {
            assert_eq!(is_prime(numbers[index]), answers[index]);
        }
    }

    #[test]

    fn prime_factors() {
        let numbers: Vec<usize> = vec![1,2,3,4,5,6,7,8,9,10,10478678];
        let answers: Vec<Vec<(usize,usize)>> = vec![
            vec![],
            vec![(2,1)],
            vec![(3,1)],
            vec![(2,2)],
            vec![(5,1)],
            vec![(2,1),(3,1)],
            vec![(7,1)],
            vec![(2,3)],
            vec![(3,2)],
            vec![(2,1),(5,1)],
            vec![(2, 1), (7, 1), (337, 1), (2221, 1)],
        ];

        for index in 0..answers.len() {
            assert_eq!(answers[index], prime_factorization(numbers[index]));
        }
    }
}
