const FAHRENHEIT:f64 = 32.0;

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f-FAHRENHEIT)*5.0/9.0
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    c*(9.0/5.0)+FAHRENHEIT
}

fn main() {
    let mut ftemp = 28.0;

    let mut result = celsius_to_fahrenheit(ftemp);
    println!("{:.1} celsius is {:.1} fahrenheit", ftemp, result);
    ftemp = 86.0;
    result = fahrenheit_to_celsius(ftemp);
    println!("{:.1} fahrenheit is {:.1} celsius", ftemp, result);
    

    let mut counter = 0;
    loop {
        if counter == 5 {
            break;
        }
        ftemp = ftemp + 1.0;
        result = fahrenheit_to_celsius(ftemp);
        println!("fahrenheit: {:.1} to celsius: {:.1}", ftemp, result);
        counter = counter + 1;
    }

}









//const FAHRENHEIT:f64 = 32.0;

/*fn fahrenheit_to_celsius(f: f64) -> f64{
//    let result = (f-FAHRENHEIT)*5.0/9.0;
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
}*/