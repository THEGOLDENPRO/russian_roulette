use std::io;
use std::process;
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();

    println!("Press [Enter] to shoot!");

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line!");
        
        let random_num = rng.gen_range(1, 7);

        if random_num == 6 {
            println!("{}BANG!{}", "\u{001b}[31;1m", "\u{001b}[0m");
            process::exit(0);
        } else {
            println!("{}Fewww, your safe :){}", "\u{001b}[38;5;51m", "\u{001b}[0m");
        }

        println!("[Enter] to pull trigger!");
    }
}
