/*
                            lib_utils

                Miscellaneous utility functions

VERSION = "0.0.5";
AUTHOR = "John T. Reagan";
LICENSE = "MIT";
LICENSE_URL = "https://opensource.org/licenses/MIT";
REPOSITORY = "https://github.com/jtreagan/lib_utils";
COPYRIGHT = "Copyright (c) 2025 John T. Reagan";

*/  // Credits


pub mod utilities {
    /* ---------------------- quick_editor() --------------------------

    -- Small, basics-only editor for quickly gathering some text.
    -- Things to add:
        -- Starter text disappear on typing.
        -- Cursor flashing to show position.
        -- Window resize features.
        -- Format the window label.
        -- Quick keys for save and close.
        -- Set font and font size to something better than courier.
        -- Right click copy, cut, paste menu.  I believe right now
                those features work by ctrl-key off of the keyboard.

   -----------------------  Example   -------------------------

    fn main() {
        let quest_text = quick_editor("Passing this on to the function, this is some starter text for whatever it is you are wanting to write.  I'm purposely making it really long to test the word wrap feature. \n");

        println!("The returned text is: \n\n {} \n", quest_text);
    }

   ---------------------------------------------------------- */

    use std::{fs::File, io::Read, rc::Rc, cell::RefCell, io};
    use std::io::Write;


    pub fn util_read_file_to_string_refcell(fname: &Rc<RefCell<String>>) -> String {
        // TODO: Do you really want this function dependent on a RefCell?
        //          Maybe do conversion to string before calling it?
        // TODO: Move this function to the  lib_file  library.

        let usefname = fname.borrow().clone();

        let mut file = File::open(usefname.as_str()).expect("Can't open file!");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Oops!  Cant read file...");
        contents
    }

    pub fn util_read_print_to_term(fname: String) {
        let mut file = File::open(fname.as_str()).expect("Can't open file!");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Oops!  Cant read file...");

        println!("{}", contents);
    }

    pub fn util_concat_strvec_flag(stringvec: &mut Vec<String>, flag: char) -> String {
        // Concatenates a vector of Strings. Places a flag char between pieces.

        let mut i = 1;
        while i < stringvec.len() {
            stringvec[i] = format!("{}{}{}", flag, stringvec[i], flag);
            i += 2;
        }
        let newstring = stringvec.join("");
        newstring
    }

    pub fn util_concat_strvec(stringvec: &Vec<String>) -> String {
        let mut newstring: String = String::new();

        for item in stringvec {
            newstring = format!("{}{}", newstring, item);
            if !newstring.ends_with(' ') {
                newstring = format!("{}{}", newstring, ' ');
            }
        }
        newstring
    }

    pub fn util_get_lastchar(newstr: &String) -> Option<char> {
        let last_char = newstr.chars().nth(newstr.len() - 1);
        last_char
    }

    pub fn util_flaggedtxt_2vec(txtstr: &String, flag: char) -> Vec<&str> {
        // In general use '§' as the flag.
        let usestring = txtstr.trim();
        let txtvec: Vec<&str> = usestring.split(flag).collect();
        let mut between_flags_vec: Vec<&str> = Vec::new();

        let mut i = 1;
        while i < txtvec.len() {
            between_flags_vec.push(txtvec[i]);
            i += 2;
        }

        between_flags_vec
    }

    pub fn util_wait_for_enter() {
        println!("Press 'Enter' to continue... \n");
        io::stdout().flush().unwrap();  // Flush stdout to ensure the message is displayed immediately

        // Read a single byte from stdin
        let mut buffer = [0u8; 1];
        io::stdin().read_exact(&mut buffer).unwrap();
    }

    pub fn util_longest_string_in_vec(stringvec: &Vec<String>) -> usize {
        let mut longest_string = 0;
        for item in stringvec {
            if item.len() > longest_string {
                longest_string = item.len();
            }
        }
        longest_string
    }



    /*

        pub fn util_clear_terminal() {
            print!("{}[2J", 27 as char);
            io::stdout().flush().unwrap();
            print!("{}[1;1H", 27 as char); // Move the cursor to the top-left corner
        }


        pub fn divide_string_tovec(sentences: &String, flag: char) -> Vec<String> {
            let tempstore: Vec<&str> = sentences.split(flag).collect();

            // Convert tempstore to vec of Strings.
            let sntnc2vec: Vec<String> = tempstore
                .into_iter()
                .map(|s| s.to_string())
                .collect();
            sntnc2vec
        }

     */  // util_clear_terminal()  &  divide_string_tovec()  functions.

} // End utilities module.

pub mod misc {

    use dialoguer::Select;

    /* ~~~~~~~~~~~~~~  Menu Function Documentaion  ~~~~~~~~~~~~~~~

----------------------------------
Requires the following in the Cargo.toml file:

[dependencies]
dialoguer = { version = "0.11.0", features = ["completion"] }
console = "0.15.8"

----------------------------------

Example:

fn main() {

let opts: Vec<String> = vec!["Activity 1".to_string(),
            "Activity 2".to_string(),
            "Activity 3".to_string(),
            "Activity 4".to_string(),
            "Activity 5".to_string(),
            "Activity 6".to_string(),
            "Activity 7".to_string(),];

let choice = activity_menu(&opts, "This is a prompt);

println!("\n You chose choice # {}. \n", choice );
}

~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */  // Example for using  activity_menu()  function.

    pub fn activity_menu(opts: &Vec<String>, prompt: &str) -> usize {
        // For a terminal-based menu this works pretty good.  The  opts
        //      vector contains the menu choices.
        let choice: usize;

        let index = Select::new()
            .with_prompt(prompt)
            .report(false)
            .items(opts)
            .default(0)
            .interact();

        match index {
            Ok(index) => { choice = index + 1; },
            Err(error) => panic!("Bad return from activity menu:  {}.", error),
        };
        choice
    }
} // End of misc module

pub mod input_utilities {
    // Note that these are all terminal-based functions that don't
    //      require a GUI.

    use std::io;
    use std::io::Write;
    use std::str::FromStr;

    pub fn input_num_prompt_range(prompt: &str, min: i64, max: i64) -> i64 {
        // Changed in choice_menu.rs
        // use std::io::{self, Write};

        loop {
            print!("{}", prompt);
            let _ = io::stdout().flush().expect("Failed to flush stdout.");

            let mut input = String::new();

            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read input.");

            let input = match input.trim().parse() {
                Ok(parsed_input) => parsed_input,
                Err(_) => continue,
            };

            if (min..=max).contains(&input) {
                return input;
            } else {
                println!("\n That choice isn't allowed!  \n");
                continue;
            };
        }
    }

    pub fn input_num_prompt<U: FromStr>(prompt: &str) -> U {
        //use std::io::{self, Write};

        loop {
            print!("{}", prompt);
            let _ = io::stdout().flush().expect("Failed to flush stdout.");

            let mut input = String::new();

            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read input.");

            let input = match input.trim().parse::<U>() {
                Ok(parsed_input) => parsed_input,
                Err(_) => continue,
            };

            return input;
        }
    }

    pub fn input_string_prompt(prompt: &str) -> String {
        //use std::io::{self, Write};

        print!("{}", prompt);
        let _ = io::stdout().flush().expect("Failed to flush stdout.");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    }

    pub fn input_char_prompt(message: &str) -> char {
        let word = input_string_prompt(message);
        let mut letter = word.chars();
        let character = letter.next().unwrap();

        character
    }

    pub fn input_bool_prompt(message: &str) -> bool {
        let mut value: String;
        let truth_value: bool;

        loop {
            value = input_string_prompt(message).to_uppercase();
            match value.as_str() {
                "YES" => {
                    truth_value = true;
                    break;
                }
                "Y" => {
                    truth_value = true;
                    break;
                }
                "NO" => {
                    truth_value = false;
                    break;
                }
                "N" => {
                    truth_value = false;
                    break;
                }
                "TRUE" => {
                    truth_value = true;
                    break;
                }
                "T" => {
                    truth_value = true;
                    break;
                }
                "FALSE" => {
                    truth_value = false;
                    break;
                }
                "F" => {
                    truth_value = false;
                    break;
                }
                _ => {
                    println!(
                        "Please enter one of  yes, Yes, y, Y, no, No, n, N, true, t,
            false, f, True, T, False, or F"
                    );
                }
            };
        }
        truth_value
    }

} // End input_utilities module

pub mod vec {
    use rand::Rng;

    pub fn vec_random_choice<T>(list: &Vec<T>) -> Option<(&T, usize)> {
        if list.is_empty() {
            return None;
        }

        let index = rand::thread_rng().gen_range(0..list.len());
        Some((&list[index], index))
    }

    pub fn vec_string_to_str(list: &Vec<String>) -> Vec<&str> {
        let mut uselist: Vec<&str> = Vec::new();
        let mut i = 0;

        while i < list.len() {
            uselist.push(&list[i].as_str());
            i += 1;
        }

        uselist
    }

    pub fn vec_longest_str(v: &Vec<String>) -> Option<&String> {
        v.iter().max_by_key(|x| x.len())
    }

    /* ---------------------  Example for using vec_longest_str()
        fn main() {
            let words = vec!["a limerick".to_string(), "vall leaves".to_string(), "snow".to_string(), "flowers blue".to_string(), "A really nice sunset".to_string()];
            let longest = vec_longest_str(&words);

            match longest {
                Some(word) => println!("\nThe longest word is: {}/n", word),
                None => println!("The vector is empty."),
            }
        }

     */  //Example for using vec_longest_str()

} // End vec module