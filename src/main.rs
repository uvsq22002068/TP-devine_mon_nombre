use std::io;
use rand::Rng;

fn main() -> io::Result<()>{
    println!("Devine mon nombre ! 
    
Saisissez votre proposition :");

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    let rng = rand::thread_rng().gen_range(1..101);

    println!("valeur entree : {}", buffer.trim());
    println!("valeur aléatoire : {rng}");
    
    Ok(())
}