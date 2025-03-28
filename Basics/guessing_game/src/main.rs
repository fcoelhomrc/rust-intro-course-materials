use rand;
use std::io;

fn get_user_input() -> u32 {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input: &str = input.trim();
    input.parse::<u32>().unwrap()  // typecast to u32
}

fn main() {
    // todo!("Implementar um jogo de adivinhar onde o jogador tenta adivinhar um número aleatório decidido pelo computador")

    loop {

        let correct: u32 = rand::random_range(0..=100);
        println!("Guess a number between 0 and 100!");

        let mut guess: u32 = get_user_input();

        while !(guess == correct) {
            if guess > correct {
                println!("Too high!");
                guess = get_user_input();
            } else {
                println!("Too low!");
                guess = get_user_input();
            }
        }

        println!("You win!");
    }
}
