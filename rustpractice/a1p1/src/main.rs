const FAHRENHEIT:f64 = 32.0;

fn fahrenheit_to_celsius(f: f64) -> f64{
    let result = (f-FAHRENHEIT)*5.0/9.0;
    return result;
}
//fn celsius_to_fahrenheit(c: f64) -> f64{
//    let result = (c*9.0/5.0)+FAHRENHEIT;
//   return result;
//}

fn main() {
    let mut x = 32.0;
    println!("{}", fahrenheit_to_celsius(x));

    let mut counter = 0;
    while counter != 5{
        println!("{}", x);
        println!("{}", fahrenheit_to_celsius(x) as i32);
        //println!("{}", celsius_to_fahrenheit(x) as i32);
        x += 1.0;
        counter += 1;
    }
}