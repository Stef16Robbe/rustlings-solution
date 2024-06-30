// quiz2.rs
//
// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums
//
// Let's build a little machine in the form of a function. As input, we're going
// to give a list of strings and commands. These commands determine what action
// is going to be applied to the string. It can either be:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times
// The exact form of this will be:
// - The input is going to be a Vector of a 2-length tuple,
//   the first element is the string, the second one is the command.
// - The output element is going to be a Vector of strings.
//
// No hints this time!

pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

struct Dinges(String, Command);

mod my_module {
    use crate::Dinges;

    use super::Command;

    pub fn transformer(input: Vec<Dinges>) -> Vec<String> {
        let mut output: Vec<String> = vec![];
        for Dinges(string, command) in input.iter() {
            match command {
                Command::Uppercase => output.push(string.to_uppercase()),
                Command::Trim => output.push(string.trim().to_owned()),
                Command::Append(times) => {
                    let mut new = string.clone();
                    for _ in 0..*times {
                        new.push_str("bar");
                    }

                    output.push(new);
                }
            }
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use super::Command;
    use crate::{my_module::transformer, Dinges};

    #[test]
    fn it_works() {
        let output = transformer(vec![
            Dinges("hello".into(), Command::Uppercase),
            Dinges(" all roads lead to rome! ".into(), Command::Trim),
            Dinges("foo".into(), Command::Append(1)),
            Dinges("bar".into(), Command::Append(5)),
        ]);
        assert_eq!(output[0], "HELLO");
        assert_eq!(output[1], "all roads lead to rome!");
        assert_eq!(output[2], "foobar");
        assert_eq!(output[3], "barbarbarbarbarbar");
    }
}
