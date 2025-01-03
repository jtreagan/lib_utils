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

    use std::{fs::File, io::Read, rc::Rc, cell::RefCell};

    // TODO: Do you really want this function dependent on a RefCell?
    //          Maybe do conversion to string before calling it?
    pub fn util_read_file_to_string(fname: &Rc<RefCell<String>>) -> String {
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

// Concatenates a vector of Strings. Places a flag char between pieces.
    pub fn util_concat_strvec_flag(stringvec: &mut Vec<String>, flag: char) -> String {
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

// In general use '§' as the flag.
    pub fn util_flaggedtxt_2vec(txtstr: &String, flag: char) -> Vec<&str> {
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
