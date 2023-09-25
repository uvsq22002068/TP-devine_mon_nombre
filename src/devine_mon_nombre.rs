use std::io;

fn main() -> io::Result<()>{
    println!("Devine mon nombre ! 
    
Saisissez votre proposition :");

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;

    println!("valeur entree : {}", buffer.trim());

    Ok(())
}