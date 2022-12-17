//this function silently fails to format
pub async fn async_main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> 
{
    println!("complete");
    Ok(())
}