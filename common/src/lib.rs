mod errors;

pub async fn add_end_of_msg(msg: &mut String) -> Result<&mut String, errors::MyError> {
    msg.push_str("\r\n");
    Ok(msg)
}

pub async fn remove_end_of_msg(msg: String) -> Result<String, errors::MyError> {
    let trimmed_msg = msg.trim_end_matches("\r\n");
    Ok(trimmed_msg.to_string())
}


pub async fn has_end_of_msg(msg: &mut String) -> Result<bool, errors::MyError> {
    let flag = msg.ends_with("\r\n");
    Ok(flag)
}

#[cfg(test)]
mod tests {
    use crate::errors::MyError;
    use super::*;

    #[tokio::test]
    async fn test_add_end_of_msg() -> Result<(), MyError>{
        let mut input = "My first line".to_string();
        let result = add_end_of_msg(&mut input).await?;
        assert_eq!(result, &mut "My first line\r\n".to_string());
        Ok(())
    }
}
