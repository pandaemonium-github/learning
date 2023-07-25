
use anyhow::Result;
use std::io;

fn main() -> Result<()>{
    loop {
        println!("Enter a number: ");
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let number: u32 = input.trim().parse()?;
        println!("{}", raindrops::sound(number));
    }
}
