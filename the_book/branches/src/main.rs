fn main() {
    {
        let number = 3;

        if number < 5 {
            println!("number is less than 5");
        } else if number < 10  {
            println!("number is less than 10");
        } else {
            println!("number is greater than or equal 10");
        }
    }

    {
        let condition = true;

        let number = if condition { 5 } else { 6 };

        println!("The value of number is: {number}");
    }
}
