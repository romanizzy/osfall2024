fn is_even(n: i32) -> bool{
    if (n%2)==0{
        return true;
    }
    else{
        return false;
    }
}

fn main() {
    let numbers: [i32; 10] = [10, 15, 21, 25, 30, 40, 66, 75, 95, 100];
    
    let mut even_odd = false;
    for idx in 0..numbers.len(){
        even_odd = is_even(numbers[idx]);
        if even_odd == true{
            println!("{} is even", numbers[idx]);
        }
        else{
            println!("{} is odd", numbers[idx]);
        }

        if (numbers[idx]%3) == 0 && (numbers[idx]%5) == 0{
            println!("FizzBuzz");
        }
        else if numbers[idx]%3==0{
            println!("Fizz");
        }
        else{
            println!("Buzz");
        }
    }

    let mut i = 0;
    let mut sum = 0;
    while i<10{
        sum += numbers[i];
        i+= 1;
    }
    println!("The sum is {}", sum);

    let mut largest = 0;
    for idx in 0..numbers.len(){
        if numbers[idx] > largest{
            largest = numbers[idx];
        }
    }
    println!("The largest number is {}", largest);

}
