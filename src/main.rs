use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() -> io::Result<()> {
    println!(
        "Devine mon nombre ! 
    
Saisissez votre proposition :"
    );

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    let rng = rand::thread_rng().gen_range(1..101);

    println!("valeur entree : {}", buffer.trim());
    println!("valeur aléatoire : {rng}");

    let n = buffer.trim().parse::<u32>().unwrap();

    let message = match n.cmp(&rng) {
        Ordering::Equal => "le nombre entré est égal au nombre aléatoire",
        Ordering::Greater => "le nombre entré est plus grand que le nombre aléatoire",
        Ordering::Less => "le nombre entré est plus petit que le nombre aléatoire",
    };

    println!("{message}");

    Ok(())
}
