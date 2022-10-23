use std::io;
use rand::seq::SliceRandom;


#[derive(PartialEq)]
enum Message {
    Won,
    Lost,
    Tie,
}

#[derive(Debug, PartialEq)]
enum Choice {
    Rock,
    Paper,
    Scissors,
    NoChoice,
}

fn main() {
    println!("Welcome to Rock, Paper, Scissors!");

    let possible_choices = [Choice::Rock, Choice::Paper, Choice::Scissors];

    loop {
        //opponent chooses random choice
        let mut rng = rand::thread_rng();
        let mut rand_choice = possible_choices.choose(&mut rng);
        
        //user gives input for choice
        let mut choice = String::new();
        println!("(r)ock, (p)aper, (s)cissors");
        io::stdin().read_line(&mut choice).expect("Couldn't read line.");
        let choice = choice.trim();

        let choice = match choice {
            "r" => Choice::Rock,
            "p" => Choice::Paper,
            "s" => Choice::Scissors,
            _ => {
                println!("Invalid input. ");
                Choice::NoChoice
            }
        };

        //Repetitive code to check the round's end result
        if choice == Choice::Rock {
            if rand_choice == Some(&Choice::Rock) {
                print_message(&Message::Tie, rand_choice);
            }
            else if rand_choice == Some(&Choice::Paper) {
                print_message(&Message::Lost, rand_choice);
            }
            else if rand_choice == Some(&Choice::Scissors) {
                print_message(&Message::Won, rand_choice);
            }
        }
        else if choice == Choice::Paper {
            if rand_choice == Some(&Choice::Paper) {
                print_message(&Message::Tie, rand_choice);
            }
            else if rand_choice == Some(&Choice::Scissors) {
                print_message(&Message::Lost, rand_choice);
            }
            else if rand_choice == Some(&Choice::Rock) {
                print_message(&Message::Won, rand_choice);
            }
        
        }
        else if choice == Choice::Scissors {
            if rand_choice == Some(&Choice::Scissors) {
                print_message(&Message::Tie, rand_choice);
            }
            else if rand_choice == Some(&Choice::Rock) {
                print_message(&Message::Lost, rand_choice);
            }
            else if rand_choice == Some(&Choice::Paper) {
                print_message(&Message::Won, rand_choice);
            }
        
        }

    }
}

fn print_message(msg_type: &Message, ai_choice: Option<&Choice>) {
    //Change ai_choice from an Option<Choice> to just Choice
    let ai_choice = match ai_choice {
        Some(Choice) => Choice,
        None       => &Choice::NoChoice,
    };
    //Check message type and print it.
    match msg_type {
        Message::Won => {
            println!("You beat your opponent. They chose {:?}", ai_choice);
        },
        Message::Lost => {
            println!("Your opponent beat you. They chose {:?}", ai_choice);
        },
        Message::Tie => {
            println!("You have tied. Both you and your opponent chose {:?}", ai_choice);
        }
    }
}
