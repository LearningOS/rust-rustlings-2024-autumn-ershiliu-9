pub enum Command {
    Uppercase,
    Trim,
    Append(String), // Changed to accept a String
}

mod my_module {
    use super::Command;

    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        let mut output: Vec<String> = vec![];
        for (string, command) in input {
            let transformed = match command {
                Command::Uppercase => string.to_uppercase(),
                Command::Trim => string.trim().to_string(),
                Command::Append(app_str) => {
                    println!("Appending '{}' to '{}'", app_str, string);
                    format!("{}{}", string, app_str)
                }
            };
            output.push(transformed);
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use super::my_module::transformer;
    use super::Command;

    #[test]
    fn it_works() {
        let output = transformer(vec![
            ("hello".into(), Command::Uppercase),
            (" all roads lead to rome! ".into(), Command::Trim),
            ("foo".into(), Command::Append("bar".into())), // Pass the string to append
            ("bar".into(), Command::Append("barbarbarbarbarbar".into())), // Pass the string to append
        ]);
        println!("Output: {:?}", output);
        assert_eq!(output[0], "HELLO");
        assert_eq!(output[1], "all roads lead to rome!");
        assert_eq!(output[2], "foobar");
        assert_eq!(output[3], "barbarbarbarbarbarbar");
    }
}
