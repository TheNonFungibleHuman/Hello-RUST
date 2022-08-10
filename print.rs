pub fn run() {
    // Print to console 
    println!("Hello from the print.rs file");

    //Basic formatting
    println!("{} is from {}", "Hanif", "Mars");

    //Postitional Arguments
    println!("{0} is from {1} and {0} likes to {2}", "Hanif", "Mars", "code" );

    //Named arguments 
    println!("{name} likes to play {activity }", name = "Hanif", activity = "Football" );

    // Placeholde Traits 
    println!("Binary: {:b} Hex: {:x} Octo: {:o}", 10, 10, 10 );
    println!("Binary: {:b} Hex: {:x} Octo: {:o}", 22, 24, 36 );

    //Placeholder for debug traits 
    println!("{:?}", (12, true, "hello") );

    // Basic maths 
    println!("10 + 10 = {}", 10 + 10);

}