mod domains;
mod errors;
mod controllers;

#[tokio::main]
async fn main() {
	controllers::telegram::main().await;
}
