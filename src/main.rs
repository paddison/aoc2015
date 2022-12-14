use std::time::Instant;

mod d1;
mod d2;
mod d3;
mod d4;
mod d5;
mod d6;
mod d7;
mod d8;
mod d9;
mod d10;
mod d11;
mod d12;
mod d13;
mod d14;
mod d15;
mod d16;
mod d17;
mod d18;
mod d19;
mod d20;
mod d21;
mod d22;
mod d23;
mod d24;
mod d25;

fn main() {
    let start_all = Instant::now();

    let start = Instant::now();
    let result = d1::get_solution_1();
    let end = start.elapsed().as_micros();
    println!("Day  1.1\t{}us\t{}", end, result);

    let start = Instant::now();
    let result = d1::get_solution_2();
    let end = start.elapsed().as_micros();
    println!("Day  1.2\t{}us\t{}", end, result);

    let start = Instant::now();
    let result = d2::get_solution_1();
    let end = start.elapsed().as_micros();
    println!("Day  2.1\t{}us\t{}", end, result);

    let start = Instant::now();
    let result = d2::get_solution_2();
    let end = start.elapsed().as_micros();
    println!("Day  2.2\t{}us\t{}", end, result);

    let start = Instant::now();
    let result = d3::get_solution_1();
    let end = start.elapsed().as_micros();
    println!("Day  3.1\t{}us\t{}", end, result);

    let start = Instant::now();
    let result = d3::get_solution_2();
    let end = start.elapsed().as_micros();
    println!("Day  3.2\t{}us\t{}", end, result);

    let start = Instant::now();
    let result = d4::get_solution_1();
    let end = start.elapsed().as_micros();
    println!("Day  4.1\t{}us\t{}", end, result);

    let start = Instant::now();
    let result = d4::get_solution_2(); 
    let end = start.elapsed().as_micros();
    println!("Day  4.2\t{}us\t{}", end, result);

    let start = Instant::now();
    let result = d5::get_solution_1(); 
    let end = start.elapsed().as_micros();
    println!("Day  5.1\t{}us\t{}", end, result);

    let start = Instant::now();
    let result = d5::get_solution_2(); 
    let end = start.elapsed().as_micros();
    println!("Day  5.2\t{}us\t{}", end, result);

    let start = Instant::now();
    let result = d6::get_solution_1(); 
    let end = start.elapsed().as_micros();
    println!("Day  6.1\t{}us\t{}", end, result);

    let start = Instant::now();
    let result = d6::get_solution_2(); 
    let end = start.elapsed().as_micros();
    println!("Day  6.2\t{}us\t{}", end, result);

    let start = Instant::now();
    let result = d7::get_solution_1(); 
    let end = start.elapsed().as_micros();
    println!("Day  7.1\t{}us\t{}", end, result);

    let start = Instant::now();
    let result = d7::get_solution_2(); 
    let end = start.elapsed().as_micros();
    println!("Day  7.2\t{}us\t{}", end, result);

    let start = Instant::now();
    let result = d8::get_solution_1(); 
    let end = start.elapsed().as_micros();
    println!("Day  8.1\t{}us\t{}", end, result);

    let start = Instant::now();
    let result = d8::get_solution_2(); 
    let end = start.elapsed().as_micros();
    println!("Day  8.2\t{}us\t{}", end, result);

    let start = Instant::now();
    let result = d9::get_solution_1(); 
    let end = start.elapsed().as_micros();
    println!("Day  9.1\t{}us\t{}", end, result);

    let start = Instant::now();
    let result = d9::get_solution_2(); 
    let end = start.elapsed().as_micros();
    println!("Day  9.2\t{}us\t{}", end, result);

    let start = Instant::now();
    let result = d10::get_solution_1(); 
    let end = start.elapsed().as_micros();
    println!("Day 10.1\t{}us\t{}", end, result);

    let start = Instant::now();
    let result = d10::get_solution_2(); 
    let end = start.elapsed().as_micros();
    println!("Day 10.2\t{}us\t{}", end, result);

    let start = Instant::now();
    let result = d11::get_solution_1(); 
    let end = start.elapsed().as_micros();
    println!("Day 11.1\t{}us\t{}", end, result);

    let start = Instant::now();
    let result = d11::get_solution_2(); 
    let end = start.elapsed().as_micros();
    println!("Day 11.2\t{}us\t{}", end, result);

    let start = Instant::now();
    let result = d12::get_solution_1(); 
    let end = start.elapsed().as_micros();
    println!("Day 12.1\t{}us\t{}", end, result);

    let start = Instant::now();
    let result = d12::get_solution_2(); 
    let end = start.elapsed().as_micros();
    println!("Day 12.2\t{}us\t{}", end, result);

    let start = Instant::now();
    let result = d13::get_solution_1(); 
    let end = start.elapsed().as_micros();
    println!("Day 13.1\t{}us\t{}", end, result);

    let start = Instant::now();
    let result = d13::get_solution_2(); 
    let end = start.elapsed().as_micros();
    println!("Day 13.2\t{}us\t{}", end, result);

    let start = Instant::now();
    let result = d14::get_solution_1(); 
    let end = start.elapsed().as_micros();
    println!("Day 14.1\t{}us\t{}", end, result);

    let start = Instant::now();
    let result = d14::get_solution_2(); 
    let end = start.elapsed().as_micros();
    println!("Day 14.2\t{}us\t{}", end, result);

    let start = Instant::now();
    let result = d15::get_solution_1(); 
    let end = start.elapsed().as_micros();
    println!("Day 15.1\t{}us\t{}", end, result);

    let start = Instant::now();
    let result = d15::get_solution_2(); 
    let end = start.elapsed().as_micros();
    println!("Day 15.2\t{}us\t{}", end, result);

    let start = Instant::now();
    let result = d16::get_solution_1(); 
    let end = start.elapsed().as_micros();
    println!("Day 16.1\t{}us\t{}", end, result);

    let start = Instant::now();
    let result = d16::get_solution_2(); 
    let end = start.elapsed().as_micros();
    println!("Day 16.2\t{}us\t{}", end, result);

    let start = Instant::now();
    let result = d17::get_solution_1(); 
    let end = start.elapsed().as_micros();
    println!("Day 17.1\t{}us\t{}", end, result);

    let start = Instant::now();
    let result = d17::get_solution_2(); 
    let end = start.elapsed().as_micros();
    println!("Day 17.2\t{}us\t{}", end, result);

    let start = Instant::now();
    let result = d18::get_solution_1(); 
    let end = start.elapsed().as_micros();
    println!("Day 18.1\t{}us\t{}", end, result);

    let start = Instant::now();
    let result = d18::get_solution_2(); 
    let end = start.elapsed().as_micros();
    println!("Day 18.2\t{}us\t{}", end, result);

    let start = Instant::now();
    let result = d19::get_solution_1(); 
    let end = start.elapsed().as_micros();
    println!("Day 19.1\t{}us\t{}", end, result);

    let start = Instant::now();
    let result = d19::get_solution_2(); 
    let end = start.elapsed().as_micros();
    println!("Day 19.2\t{}us\t{}", end, result);

    let start = Instant::now();
    let result = d20::get_solution_1(); 
    let end = start.elapsed().as_micros();
    println!("Day 20.1\t{}us\t{}", end, result);

    let start = Instant::now();
    let result = d20::get_solution_2(); 
    let end = start.elapsed().as_micros();
    println!("Day 20.2\t{}us\t{}", end, result);

    let start = Instant::now();
    let result = d21::get_solution_1(); 
    let end = start.elapsed().as_micros();
    println!("Day 21.1\t{}us\t{}", end, result);

    let start = Instant::now();
    let result = d21::get_solution_2(); 
    let end = start.elapsed().as_micros();
    println!("Day 21.2\t{}us\t{}", end, result);

    let start = Instant::now();
    let result = d22::get_solution_1(); 
    let end = start.elapsed().as_micros();
    println!("Day 22.1\t{}us\t{}", end, result);

    let start = Instant::now();
    let result = d22::get_solution_2(); 
    let end = start.elapsed().as_micros();
    println!("Day 22.2\t{}us\t{}", end, result);

    let start = Instant::now();
    let result = d23::get_solution_1(); 
    let end = start.elapsed().as_micros();
    println!("Day 23.1\t{}us\t{}", end, result);

    let start = Instant::now();
    let result = d23::get_solution_2(); 
    let end = start.elapsed().as_micros();
    println!("Day 23.2\t{}us\t{}", end, result);

    let start = Instant::now();
    let result = d24::get_solution_1(); 
    let end = start.elapsed().as_micros();
    println!("Day 24.1\t{}us\t{}", end, result);

    let start = Instant::now();
    let result = d24::get_solution_2(); 
    let end = start.elapsed().as_micros();
    println!("Day 24.2\t{}us\t{}", end, result);

    let start = Instant::now();
    let result = d25::get_solution_1(); 
    let end = start.elapsed().as_nanos();
    println!("Day 25.1\t{}ns\t{}", end, result);

    let end_all = start_all.elapsed().as_millis();
    println!("\nTotal runtime:\t{}ms (excluding day 4.1, 4.2, 10.2, 20.1 and 20.2", end_all);
}
