fn convert_c_to_f(c: f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0
}

fn count_seconds(hours : u32, minutes : u8, seconds : u8) -> u32 {
    let total_minutes = hours * 60 + minutes as u32;
    total_minutes * 60 + seconds as u32
}

fn main() {
    println!("Hello World");

    let c = 100.0;
    let f = convert_c_to_f(c);
    println!("F = {}", f);

    let total = count_seconds(1000, 50, 30);
    println!("Seconds = {}", total);

    let numbers = [7, 4, 9];
    for number in numbers {
        println!("{}", number*number);
    }

    for index in 0..numbers.len() {
        let number1 = numbers[index];
        let number2 = numbers.get(index).unwrap();
        println!("{} = {}", number1, number2);
    }

}
