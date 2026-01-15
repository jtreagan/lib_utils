//!# Miscellaneous utility functions
//!
//! I've used the functions in the modules of this crate
//! in several different projects,
//! which is why I've kept them together in a separate crate.
//! Their greatest weakness is poor error handling, so keep that
//! in mind if you choose to use them.  By the way, I need help getting
//! those weaknesses corrected, so if you feel like taking that on,
//! please check out the issues tab in this crate's repository.
//!
//!   * VERSION = "0.0.5";
//!   * AUTHOR = "John T. Reagan";
//!   * LICENSE = "MIT";
//!   * LICENSE_URL = "<https://opensource.org/licenses/MIT>";
//!   * REPOSITORY = "<https://github.com/jtreagan/lib_utils>";
//!   * COPYRIGHT = "Copyright (c) 2025 John T. Reagan";
//!


/// Miscellaneous utility functions
///
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

    use dialoguer::Select;
    use std::io::Write;
    use std::{io, io::Read};

    /// Concatenates a vector of Strings. Places a flag char between pieces.
    ///
    pub fn util_concat_strvec_flag(stringvec: &mut Vec<String>, flag: char) -> String {
        let mut i = 1;
        while i < stringvec.len() {
            stringvec[i] = format!("{}{}{}", flag, stringvec[i], flag);
            i += 2;
        }
        let newstring = stringvec.join("");
        newstring
    }

    /// Concatenate a vector of Strings into a single String
    /// that is then returned.
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

    /// Returns the very last character of a String.
    ///
    pub fn util_get_lastchar(newstr: &String) -> Option<char> {
        let last_char = newstr.chars().nth(newstr.len() - 1);
        last_char
    }

    /// Breaks up a String that contains flag characters, saving the sections
    /// between the flags as elements of a vector that is then returned.
    pub fn util_flaggedtxt_2vec(txtstr: &String, flag: char) -> Vec<&str> {
        //! Any character can be used as the flag.  The author generally
        //! uses '§' as the flag.
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

    /// Does just what the name says.  Waits for the user to press `Enter`
    /// before continuing execution.
    pub fn util_wait_for_enter() {
        println!("Press 'Enter' to continue... \n");
        io::stdout().flush().unwrap();  // Flush stdout to ensure the message is displayed immediately

        // Read a single byte from stdin
        let mut buffer = [0u8; 1];
        io::stdin().read_exact(&mut buffer).unwrap();
    }

    /// Use the elements of a String vector to create a terminal-based
/// numbered menu.  Returns the number of the item chosen by the user.
///
/// Requires the following in the Cargo.toml file:
///
///     [dependencies]
///     dialoguer = { version = "0.11.0", features = ["completion"]}
///
/// # Example:
///
///     fn main() {
///         let opts: Vec<String> = vec!["Activity 1".to_string(),
///                 "Activity 2".to_string(),
///                 "Activity 3".to_string(),
///                 ];
///
///         let choice = util_activity_menu(&opts, "This is a prompt. Use up and down arrows to select.  Enter to confirm.");
///
///         println!("\n You chose choice # {}. \n", choice );
///
///         match choice {
///             1 => println!("\n 11111111111"),
///             2 => println!("\n 22222222222"),
///             3 => println!("\n 33333333333"),
///         }
///     }
    pub fn util_activity_menu(opts: &Vec<String>, prompt: &str) -> usize {
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


    /// Prints elements of a given slice in pairs,formatted in a 
    /// column-like structure. 
    ///
    /// # Parameters
    /// - `numvec`: A reference to a slice containing elements that implement 
    ///     the `Display` trait.
    ///
    /// # Behavior
    /// - Iterates through the slice `numvec` and prints its elements in pairs.
    /// - Each pair is formatted as `element1, element2` and printed in a 
    ///     separate line.
    /// - If the number of elements in the slice is odd the final remaining 
    ///     element (unpaired) is printed on its own line.
    ///
    /// # Example
    /// ```
    /// let numbers = vec![1, 2, 3, 4, 5];
    /// print_pairs_col(&numbers);
    /// // Output:
    /// //
    /// //          1, 2
    /// //          3, 4
    /// //          5
    /// //
    /// let fruits = vec!["apple", "banana", "orange"];
    /// print_pairs_col(&fruits);
    /// // Output:
    /// //
    /// //          apple, banana
    /// //          orange
    /// //
    /// ```
    ///
    /// # Notes
    /// - The function ensures proper formatting by including additional spacing 
    ///     and newline characters.
    /// - Elements must implement the `Display` trait to allow formatting 
    ///     using `format!` and `println!`.
    ///
    /// # Complexity
    /// - Time complexity: O(n), where `n` is the number of elements in the slice.
    /// - Space complexity: O(1), as it operates directly on the input slice 
    ///     without allocating additional memory.
    ///
    pub fn outpt_print_pairs_col<T: std::fmt::Display>(numvec: &[T]) {
        println!("\n");  // Pad the terminal display, for readability.

        // Loop until we reach the last element or the last pair
        let mut i = 0;
        while i + 1 < numvec.len() {
            let line = format!("\t\t\t{}, {}", numvec[i], numvec[i + 1]);
            println!("{}", line);
            i += 2;
        }

        if numvec.len() % 2 != 0 {
            // If odd number of elements, print last element.
            println!("\t\t\t{}", numvec[numvec.len() - 1]);
        }

        println!("\n");  // Pad the terminal display, for readability.
    }
    

}


/// Functions that facilitate terminal-based data input.
///
pub mod input_utilities {

    use std::io;
    use std::io::Write;
    use std::str::FromStr;

    /// Prompts user to enter an integer within a given range.
    ///
    pub fn input_num_prompt_range(prompt: &str, min: i64, max: i64) -> i64 {
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

    /// Prompts the user with a custom message to input a value, then reads and 
    /// parses the input into the specified type.
    ///
    /// # Type Parameters
    /// * `U` - The type the input will be parsed into. This type must implement
    /// the `FromStr` trait.
    ///
    /// # Parameters
    /// * `prompt` - A string slice that represents the message to display 
    /// to the user before waiting for input.
    ///
    /// # Returns
    /// * `U` - The parsed value of the user's input, converted into the 
    /// specified type.
    ///
    /// # Behavior
    /// The function repeatedly prompts the user for input until a valid 
    /// value of the specified type (`U`) is entered. If the input
    /// cannot be parsed into the desired type, the function will ignore the 
    /// invalid input and prompt the user again.
    /// 
    /// # Weaknesses
    /// Does not allow for breaking the loop until the user enters a valid value.
    ///
    /// # Panics
    /// The function will panic in the following scenarios:
    /// * If flushing `stdout` fails.
    /// * If reading the user's input from `stdin` encounters an error.
    ///
    /// # Example
    /// 
    ///     use std::str::FromStr;
    /// 
    ///     main() {
    ///          let num: i32 = input_num_prompt("Please enter a number: ");
    ///          println!("You entered: {}", num);
    ///     }
    ///
    /// # Notes
    /// Ensure that the type `U` is compatible with the format of the input expected from the user. For example, if `U` is `i32`,
    /// the user must enter valid integers.
    ///
    pub fn input_num_prompt<U: FromStr>(prompt: &str) -> U {
        //use std::io::{self, Write};
        // todo: Change to return a Result & eliminate the loop.
        //    Or, at least make a similar function that will do that.

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

    /// Prompts user to enter a String.
    ///
    pub fn input_string_prompt(prompt: &str) -> String {
        //use std::io::{self, Write};

        print!("{}", prompt);
        let _ = io::stdout().flush().expect("Failed to flush stdout.");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    }

    /// Prompts user to enter a character.
    ///
    pub fn input_char_prompt(message: &str) -> char {
        let word = input_string_prompt(message);
        let mut letter = word.chars();
        let character = letter.next().unwrap();

        character
    }

    /// Prompts user to enter a bool.  Allows for several forms of
    /// true or false responses.
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

}

/// Functions that operate on vectors.
///
pub mod vec {
    use rand::Rng;

    /// Pick an element randomly.  Return, in a tuple, the element
    /// along with the it's index in the vector.
    pub fn vec_random_choice<T>(list: &Vec<T>) -> Option<(&T, usize)> {
        if list.is_empty() {
            return None;
        }

        let index = rand::thread_rng().gen_range(0..list.len());
        Some((&list[index], index))
    }

    /// Converts a vector of Strings into a vector of &str's.
    ///
    pub fn vec_string_to_str(list: &Vec<String>) -> Vec<&str> {
        let mut uselist: Vec<&str> = Vec::new();
        let mut i = 0;

        while i < list.len() {
            uselist.push(&list[i].as_str());
            i += 1;
        }

        uselist
    }

    /// In a vector of Strings, returns the longest element.
    /// If more than one element has the longest length, the last
    /// of those long elements is returned.
    ///
    /// # Example
    ///
    ///         fn main() {
    ///             let words = vec!["a limerick".to_string(),
    ///                             "fall leaves".to_string(),
    ///                             "snow".to_string(),
    ///                             "flowers blue".to_string(),
    ///                             "A really nice sunset".to_string()
    ///                             ];
    ///             let longest = vec_longest_str(&words);
    ///
    ///             match longest {
    ///                 Some(word) => println!("\n The longest word in the vector is: {} /n", word),
    ///                 None => println!("\n The vector is empty. \n"),
    ///             }
    ///         }
    ///
    pub fn vec_longest_str(v: &Vec<String>) -> Option<&String> {
        // Using `max_by_key()` here is slick and quick, but
        // I honestly prefer the simple logic I used in the
        // `vec_longest_string_len()` function below.  Simple
        // and straightforward is, in my opinion, better coding.

        v.iter().max_by_key(|x| x.len())
    }

    /// Returns the length of the longest String in a String vector
    ///     in bytes.  Don't mistake this for pixels.
    pub fn vec_longest_str_len(stringvec: &Vec<String>) -> usize {
        let mut longest = 0;
        for item in stringvec {
            if item.len() > longest {
                longest = item.len();
            }
        }
        longest
    }
}