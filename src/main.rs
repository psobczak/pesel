use models::pesel::Pesel;

mod models;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pesel = Pesel::new(String::from("44051401458"))?;
    println!("{:?}", pesel);



    Ok(())
}


