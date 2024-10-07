use jelly::{adapters::{dependency::bfs::BreadFirstSearch, pom::pom::Pom}, core::gdp::service::{dependency_service::DependencyService, pom_service::PomService}};
use tokio;

#[tokio::main]
async fn main() {

    match DependencyService::new(
        BreadFirstSearch::new(),
        PomService::new(Pom)
    )
    .start()
    .await {
        Ok(_) => print!("POM Processed Correclty."),
        Err(e) => eprint!("An error ocurred: {}", e),
    };

}
