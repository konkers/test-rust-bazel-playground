use anyhow::{anyhow, Result};

fn main() -> Result<()> {
    Err(anyhow!("This space left errored intentionally."))
}
