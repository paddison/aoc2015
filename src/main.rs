use std::time::Instant;

mod d1;
mod d2;
mod d3;
mod d4;
mod d5;
mod d6;
mod d7;

fn main() {
    let start_all = Instant::now();
    let start = Instant::now();
    let result = d1::get_solution_1();
    let end = start.elapsed().as_micros();
    println!("Day 1.1\t{}us\t{}", end, result);

    let start = Instant::now();
    let result = d1::get_solution_2();
    let end = start.elapsed().as_micros();
    println!("Day 1.2\t{}us\t{}", end, result);

    let start = Instant::now();
    let result = d2::get_solution_1();
    let end = start.elapsed().as_micros();
    println!("Day 2.1\t{}us\t{}", end, result);

    let start = Instant::now();
    let result = d2::get_solution_2();
    let end = start.elapsed().as_micros();
    println!("Day 2.2\t{}us\t{}", end, result);

    let start = Instant::now();
    let result = d3::get_solution_1();
    let end = start.elapsed().as_micros();
    println!("Day 3.1\t{}us\t{}", end, result);

    let start = Instant::now();
    let result = d3::get_solution_2();
    let end = start.elapsed().as_micros();
    println!("Day 3.2\t{}us\t{}", end, result);

    let start = Instant::now();
    let result = d4::get_solution_1();
    let end = start.elapsed().as_micros();
    println!("Day 4.1\t{}us\t{}", end, result);

    let start = Instant::now();
    let result = d4::get_solution_2(); 
    let end = start.elapsed().as_micros();
    println!("Day 4.2\t{}us\t{}", end, result);

    let start = Instant::now();
    let result = d5::get_solution_1(); 
    let end = start.elapsed().as_micros();
    println!("Day 5.1\t{}us\t{}", end, result);

    let start = Instant::now();
    let result = d5::get_solution_2(); 
    let end = start.elapsed().as_micros();
    println!("Day 5.2\t{}us\t{}", end, result);

    let start = Instant::now();
    let result = d6::get_solution_1(); 
    let end = start.elapsed().as_micros();
    println!("Day 6.1\t{}us\t{}", end, result);

    let start = Instant::now();
    let result = d6::get_solution_2(); 
    let end = start.elapsed().as_micros();
    println!("Day 6.2\t{}us\t{}", end, result);

    let start = Instant::now();
    let result = d7::get_solution_1(); 
    let end = start.elapsed().as_micros();
    println!("Day 7.1\t{}us\t{}", end, result);

    let start = Instant::now();
    let result = d7::get_solution_2(); 
    let end = start.elapsed().as_micros();
    println!("Day 7.2\t{}us\t{}", end, result);

    let end_all = start_all.elapsed().as_micros();
    println!("Total: {}us", end_all)
}
