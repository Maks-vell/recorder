use tokio;
use recorder::app::app::App;

///Entry Point
#[tokio::main]
async fn main() {
   App::run().await;
}
