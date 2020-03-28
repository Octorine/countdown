extern crate countdown;
use countdown::letters;
use countdown::numbers;
fn main() {
    println!("Let's countdown!\n");
    let mut all_done = false;
    while !all_done {
        let mut choice = String::new();
        println!("Enter \"l\" for letters, \"n\" for numbers, or \"q\" to quit");
        std::io::stdin().read_line(&mut choice);
        let choice = choice.trim();
        if choice == "l" {
            letters::do_letters_puzzle();
        } else if choice == "n" {
            numbers::do_numbers_puzzle();
        } else if choice == "q" {
            all_done = true;
        }
    }
}
