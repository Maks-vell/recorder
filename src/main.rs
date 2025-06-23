use tokio;
use recorder::app::app::App;

#[tokio::main]
async fn main() {
   App::run().await;
}
