fn is_even(n: i32) -> bool {

    if n % 2 == 0{
        return true;
    }
    else{
        return false;
    }
    
}

fn main() {

    let arr: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 15];
    let mut largest = arr[0];
    let mut num = 0;
    let mut sum = 0;
    let mut index = 0;
    let mut index_l = 0;
   
    // for loop to iterate through the array
    for _n in 1..11{
        let number = arr[num];
        
        if number % 3 == 0 && number % 5 == 0 {
            println!("{} FizzBuzz is {}", number, is_even(number));
        }
        if number % 3 == 0 && number % 5 != 0 {
            println!("{} Fizz is {}", number, is_even(number));
        }
        if number % 5 == 0 && number % 3 != 0 {
            println!("{} Buzz is {}", number, is_even(number));
        }

        if number % 3 != 0 && number % 5 != 0{ 
            println!("{} is {}", number, is_even(number));
        }
         
        num += 1;
    }

    // while loop to find and print the sum of all numbers in the array
    while index < arr.len() {
        let number = arr[index];
        sum += number;
        index += 1;
    }
    
    // prints the sum of the array
    println!("\nThe sum of all numbers in the array is {}", sum);

    // loop to find and print the largest number in the array
    loop {
        if index_l >= arr.len() {
            break;  // Exit the loop when we have checked all elements
        }

        if arr[index_l] > largest {
            largest = arr[index_l];
        }

        index_l += 1;
    }

    // Print the largest number
    println!("\nThe largest number in the array is {}", largest);

}
