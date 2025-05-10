

use lib_utils::utilities::*;

fn main() {
    let opts: Vec<String> = vec!["Activity 1".to_string(),
                                 "Activity 2".to_string(),
                                 "Activity 3".to_string(),
    ];

    let choice = util_activity_menu(&opts, "This is a prompt.  Use up and down arrows to select.  Enter to confirm.");

    println!("\n You chose choice # {}. \n", choice);

    match choice {
        1 => println!("\n 11111111111"),
        2 => println!("\n 22222222222"),
        3 => println!("\n 33333333333"),
        _ => println!("That choice isn't allowed!"),
    }
}
