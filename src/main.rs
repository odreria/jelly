use jelly::cli::app::JellyApp;
use jelly::cli::output::JellyOutput;

#[tokio::main]
async fn main() {
    let app = JellyApp::parse_args();

    if let Err(e) = app.execute().await {
        let _ = JellyOutput::error(&format!("Error: {}", e));
        std::process::exit(1);
    }
}
