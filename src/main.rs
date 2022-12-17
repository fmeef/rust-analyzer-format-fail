// this function formats correctly
pub async fn async_main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("complete");
    Ok(())
}

fn main() {
    println!("Hello, world!");
}
