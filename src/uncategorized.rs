#![allow(unused)]
pub fn binary_exponentiation(base: usize, exponent: usize) -> usize {
    let mut base = base;
    let mut exponent = exponent;
    let mut result = 1;

    while exponent > 0 {
        if exponent & 1 == 1 {
            result *= base;
        }
        base *= base;

        exponent >>= 1;
    }
    result
}

struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<usize>,
}

impl Matrix {
    pub fn get(&self, row:usize, col: usize) -> Option<usize> {
        if row >= self.rows { return None; }
        if col >= self.cols { return None; }

        Some(self.data[self.cols * col + row])
    }
    pub fn multiply(&self, other: &Self) -> Option<Self> {
        if self.cols != other.rows { return None; }
        if self.data.len() != self.cols * self.rows || other.data.len() != other.cols * other.rows { return None; }

        let mut matrix = Matrix { rows: self.rows, cols: other.cols, data: vec![0,self.rows * other.cols]};

        !unimplemented!()

    }
}

pub fn factorial_log_n(number: usize) -> usize {
   !unimplemented!() 
}

pub fn binary_exponentiation_modulo(base: usize, exponent: usize, modulo: usize) -> usize {
    let mut base = base % modulo;
    let mut exponent = exponent;
    let mut result = 1;

    while exponent > 0 {
        if exponent & 1 == 1 {
            result *= base % modulo;
        }
        base *= base % modulo;

        exponent >>= 1;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exponentiation() {
        let mut numbers: Vec<(usize, usize)> = vec![(2, 6), (11, 2), (25, 4)];
        let mut answers: Vec<usize> = vec![64, 121, 390625];

        for index in 0..numbers.len() {
            assert_eq!(
                answers[index],
                binary_exponentiation(numbers[index].0, numbers[index].1)
            );
        }
    }

    #[test]
    fn exponentiaton_modulo() {
        let mut numbers: Vec<(usize, usize, usize)> = vec![(2, 6, 2), (11, 2, 6), (25, 4, 512)];
        let mut answers: Vec<usize> = vec![0, 1, 481];

        for index in 0..numbers.len() {
            assert_eq!(
                answers[index],
                binary_exponentiation_modulo(numbers[index].0, numbers[index].1, numbers[index].2)
            );
        }
    }
}
