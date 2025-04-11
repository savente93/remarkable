use color_eyre::eyre::Result;

pub async fn dummy() -> Result<()> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn dummy_test() -> Result<()> {
        Ok(())
    }
}
