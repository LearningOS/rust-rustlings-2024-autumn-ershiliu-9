pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    pub fn transformer(inputs: Vec<(String, Command)>) -> Vec<String> {
        let mut output: Vec<String> = Vec::new();
        for (mut string, command) in inputs {
            match command {
                Command::Uppercase => string.make_ascii_uppercase(),
                Command::Trim => string = string.trim().to_string(), // Correctly trim the string
                Command::Append(n) => string.push_str(&"bar".repeat(n)),
            }
            output.push(string);
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
            ("foo".into(), Command::Append(1)),
            ("bar".into(), Command::Append(5)),
        ]);
        assert_eq!(output[0], "HELLO");
        assert_eq!(output[1], "all roads lead to rome!");
        assert_eq!(output[2], "foobar");
        assert_eq!(output[3], "barbarbarbarbarbar");
    }
}