pub fn generate_nametag_text(name: String) -> Result<String, String> {
    if name.is_empty() {
        Err(format!("Name is null. Please input again."))
    } else {
        Ok(format!("Hi! My name is {}", name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_nametag_text_for_a_nonempty_name() {
        assert_eq!(
            generate_nametag_text("Beyoncé".to_string()),
            Ok("Hi! My name is Beyoncé".to_string())
        );
    }

    #[test]
    fn explains_why_generating_nametag_text_fails() {
        assert_eq!(
            generate_nametag_text("".to_string()),
            Err("Name is null. Please input again.".to_string())
        );
    }
}
