
const FREEZING_POINT_F: f64 = 32.0;

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - FREEZING_POINT_F) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + FREEZING_POINT_F
}


fn main() {
    
    let temp: f64 = 32.0;
    let celsius_temp = fahrenheit_to_celsius(temp);
    let mut fahrenheit_temp = celsius_to_fahrenheit(celsius_temp);
    let mut counter = 0;

    println!("{}°F is {:.2}°C", fahrenheit_temp, celsius_temp);

    fahrenheit_temp += 1.0;
    
    loop{
        println!("{}°F is {:.2}°C", fahrenheit_temp, fahrenheit_to_celsius(fahrenheit_temp));
        counter += 1;
        fahrenheit_temp += 1.0;

        if counter == 5{
            break;
        }

        
    }

}
