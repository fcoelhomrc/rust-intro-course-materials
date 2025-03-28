use std::io;


fn append_char(string : &mut String, ch : char) {
    string.push(ch);
}

fn pop_char(string : &mut String) {
    string.pop();
}

fn append_word(string : &mut String, new_string : String) {
    string.push_str(&new_string);
}

fn remove_word(string : &mut String, target : String) {
    *string = string.replace(&target, "");
    // replace returns a String, so we dereference and modify inplace
}

fn to_uppercase(string : &mut String) {
    *string = string.to_uppercase();
}

fn to_lowercase(string : &mut String) {
    *string = string.to_lowercase();
}


fn ask_user_input() -> String {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input = input.trim().to_string();
    input
}

fn main() {


    println!("Input some text");
    let mut string = ask_user_input();

    loop {
        println!("Select one option:");
        println!("1: Append a character");
        println!("2: Pop the last character");
        println!("3: Append a string");
        println!("4: Remove a string");
        println!("5: Convert to uppercase");
        println!("6: Convert to lowercase");

        // FIXME: crashes if parsing to u32 fails
        let choice = ask_user_input().parse::<u32>().unwrap();

        if choice == 1 {
            println!("Insert a char");
            let ch = ask_user_input().chars().next().unwrap();
            append_char(&mut string, ch);
        } else if choice == 2 {
            pop_char(&mut string);
        } else if choice == 3 {
            println!("Insert a string");
            let new_string = ask_user_input();
            append_word(&mut string, new_string);  // consumes new_string, modifies string in place
        } else if choice == 4 {
            println!("Insert a string");
            let target = ask_user_input();
            remove_word(&mut string, target);  // consumes target, modifies string in place
        } else if choice == 5 {
            to_uppercase(&mut string);
        } else if choice == 6 {
            to_lowercase(&mut string);
        } else {
            println!("Invalid choice");
        }
        println!("{}", string);
    }
}
