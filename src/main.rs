#[allow(unused_imports)]
use remarkable::*;

use color_eyre::eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    // ...
    Ok(())
}
