mod errors;

#[derive(Clone)]
pub struct Message {
    pub value: String,
}

impl Message {
    pub fn new(value: String) -> Message {
        Self { value }
    }

    pub fn remove_end_of_msg(&self) -> String {
        let trimmed_msg = self.value.trim_end_matches("\r\n");
        trimmed_msg.parse().unwrap()
    }

    pub fn has_end_of_msg(&self) -> bool {
        self.value.ends_with("\r\n")
    }

    pub fn add_end_of_msg(&self) -> String {
        let mut string = self.value.to_owned();
        if !(string.ends_with("\r\n")) {
            string.push_str("\r\n");
        }
        string.parse().unwrap()
    }

    pub fn to_upper_case(&self) -> String {
        let mut string = Message::remove_end_of_msg(&self);
        string = string.to_uppercase();
        string.push_str("\r\n");
        string
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::MyError;

    #[tokio::test]
    async fn test_has_end_of_msg() -> Result<(), MyError> {
        let test_string = "My first line".to_string();
        let input = Message::new(test_string);
        assert_eq!(input.has_end_of_msg(), false);
        Ok(())
    }

    #[tokio::test]
    async fn test_remove_end_of_msg() -> Result<(), MyError> {
        let test_string = "My first line".to_string();
        let input = Message::new(test_string);
        assert_eq!(input.remove_end_of_msg(), "My first line".to_string());
        Ok(())
    }

    #[tokio::test]
    async fn test_add_end_of_msg() -> Result<(), MyError> {
        let test_string = "My first line\r\n".to_string();
        let input = Message::new(test_string);
        assert_eq!(input.add_end_of_msg(), "My first line\r\n".to_string());
        Ok(())
    }
}
