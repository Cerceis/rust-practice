use num::Complex;
use std::env;
use std::str::FromStr;

fn main() {

    let arg = env::args().skip(1).next().expect("No available arguments found!");
   
    let parsed_num = f64::from_str(&arg).expect("Number please! 0w0");
    complex_square_loop(Complex { re: parsed_num, im: 0.0 });

}

fn complex_square_loop(c: Complex<f64>) {
    let mut z = Complex { re: 0.0, im: 0.0 };
    loop {
        z = z * z + c;
        println!("{:?}", z);
    }
}
