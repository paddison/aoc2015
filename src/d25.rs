static ROW: usize = 3010;
static COLUMN: usize =  3019;

static START_CODE: usize = 20151125;
static MULTIPLICANT: usize = 252533;
static DIVIDEND: usize = 33554393;

fn calculate_code_number(row: usize, column: usize) -> usize {
    // number of code can be calculated by taking the sum of all numbers until column + row - 1, and then subrtacting row - 1 from it (since rows are not zero indexed)
    ((column + row - 1) * (column + row) / 2) - (row - 1)
}

fn calculate_code(code_number: usize) -> usize {
    (START_CODE * pow_mod(MULTIPLICANT, code_number - 1, DIVIDEND)) % DIVIDEND
}

pub fn pow_mod(mut base: usize, mut exp: usize, modulo: usize) -> usize {
    if exp == 0 {
        return 1;
    }

    let mut result = 1;

    while 0 < exp {
        if exp % 2 == 1 {
            result = (result * base) % modulo;
        }
        exp >>= 1;
        base = (base * base) % modulo;     
    }

    result
}

pub fn get_solution_1() -> usize {
    calculate_code(calculate_code_number(ROW, COLUMN))
}

#[test]
fn test_calculate_code_number() {
    assert_eq!(1, calculate_code_number(1, 1));
    assert_eq!(9, calculate_code_number(2, 3));
    assert_eq!(17, calculate_code_number(5, 2));
    assert_eq!(20, calculate_code_number(2, 5));
    assert_eq!(21, calculate_code_number(1, 6));
}

#[test]
fn test_calculate_code() {
    assert_eq!(calculate_code(1), START_CODE);
    assert_eq!(calculate_code(2), 31916031);
    assert_eq!(calculate_code(3), 18749137);
    assert_eq!(calculate_code(10), 30943339);
    assert_eq!(calculate_code(19), 7981243);
}