mod domains;
mod errors;
mod telegram;

#[tokio::main]
async fn main() {
	telegram::controllers::main().await;
}
