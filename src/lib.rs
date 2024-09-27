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

    use std::fs::File;
    use std::io::Read;
    use fltk::app::{App, quit, set_font_size};
    use fltk::prelude::{DisplayExt, GroupExt, MenuExt, WidgetBase, WidgetExt};
    use fltk::{menu, text, window};
    use fltk::enums::{Color, Shortcut};
    use fltk::text::{TextBuffer, TextEditor};

    pub fn util_read_file_to_string(fname: &String) -> String {
        let mut file = File::open(fname.as_str()).expect("Can't open file!");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Oops!  Cant read file...");
        contents
    }

        // Concatenates a vector of Strings. Places a flag char between pieces.
    pub fn concat_strvec_flag(stringvec: &mut Vec<String>, flag: char) -> String {
        let mut i = 1;
        while i < stringvec.len() {
            stringvec[i] = format!("{}{}{}", flag, stringvec[i], flag);
            i += 2;
        }
        let newstring = stringvec.join("");
        newstring
    }

    pub fn concat_strvec(stringvec: &Vec<String>) -> String {
        let mut newstring: String = String::new();

        for item in stringvec {
            newstring = format!("{}{}", newstring, item);
            if !newstring.ends_with(' ') {
                newstring = format!("{}{}", newstring, ' ');
            }
        }
        newstring
    }

    pub fn get_lastchar(newstr: &String) -> Option<char> {
        let last_char = newstr.chars().nth(newstr.len() - 1);
        last_char
    }

    pub fn simple_editor(startertxt: &str, winlabel: &str) -> String {
        let edtr = App::default();
        let mut buf = text::TextBuffer::default();
        let mut win = window::Window::default().with_size(800, 300);
        set_font_size(20);
        win.set_color(Color::Blue);
        win.set_label(winlabel);
        win.make_resizable(true);

        simple_editor_menubar();

        buf.set_text(startertxt);
        let mut simped = TextEditor::default()
            .with_size(770, 222)
            .center_of_parent();

        simped.set_buffer(buf.clone());   // Clone is used here to avoid an ownership error.
        simped.wrap_mode(text::WrapMode::AtBounds, 0);
        simped.set_color(Color::White);
        simped.set_text_size(22);
        simped.set_text_color(Color::Black);

        win.end();
        win.show();

        //editor_menubar();

        edtr.run().unwrap();

        buf.text()
    }

    pub fn simple_editor_menubar() -> menu::MenuBar {

        let mut menubar = menu::MenuBar::new(0, 0, 800, 40, "");

        let quit_idx = menubar.add(
            "File/Finished\t",
            Shortcut::None,
            menu::MenuFlag::Normal,
            |_| {
                quit();
            },
        );
        menubar.at(quit_idx).unwrap().set_label_color(Color::Red);

        menubar
    }

    pub fn replace_highlighted_text(edtr: &TextEditor, buf: &mut TextBuffer, rpltxt: &str) {
        let (x, y) = match edtr.buffer().unwrap().selection_position() {
            Some(position) => position,
            None => panic!("\nError!  Could not find a cursor position in the editor.\n"),
        };

        buf.remove(x, y);                         // Remove the selected text
        buf.insert(x, rpltxt);                    // Insert new text and
        edtr.buffer().unwrap().unselect();        // Unhighlight text
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

     */

} // End utilities module.
