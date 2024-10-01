use std::fs::File;
use std::io::{self, Write, BufReader, BufRead};

// Define the Car struct
struct Car {
    make: String,
    model: String,
    year: u32,
}

impl Car {
    fn save_to_file(&self, path: &str) {
        let mut file = File::create(path).expect("Unable to create file");
        writeln!(file, "Make: {}", self.make).expect("Unable to write make");
        writeln!(file, "Model: {}", self.model).expect("Unable to write model");
        writeln!(file, "Year: {}", self.year).expect("Unable to write year");
    }

    // Load car details from a file
    fn from_file(path: &str) -> Car {
        let file = File::open(path).expect("Unable to open file");
        let reader = BufReader::new(file);
        let mut lines = reader.lines();

        let make = lines.next().unwrap().unwrap().replace("Make: ", "");
        let model = lines.next().unwrap().unwrap().replace("Model: ", "");
        let year: u32 = lines.next().unwrap().unwrap().replace("Year: ", "").parse().expect("Unable to parse year");

        Car { make, model, year }
    }
}

fn reading_from_console() -> Car {
    let mut buffer = String::new();

    print!("What make is your car? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let make = buffer.trim().to_string();
    buffer.clear();

    print!("What model is your car? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let model = buffer.trim().to_string();
    buffer.clear();

    print!("What year is your car? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let year: u32 = buffer.trim().parse().expect("Invalid year input");

    Car { make, model, year }
}

fn reading_from_file() {
    let car = Car::from_file("user_info.txt");
    println!("\nReading from file:");
    println!("Make: {}", car.make);
    println!("Model: {}", car.model);
    println!("Year: {}", car.year);
}

fn main() {
    let car = reading_from_console();
    car.save_to_file("user_info.txt");
    reading_from_file();
}
