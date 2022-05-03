use std::error::Error;

mod models;
use models::Book;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let book = reqwest::get("http://api.audnex.us/books/0593215753")
        .await?
        .json::<Book>()
        .await?;

    println!("{book:#?}");
    println!("{:#?}", book.image);
    Ok(())
}
