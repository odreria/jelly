use jelly::adapters::pom::pom::Pom;
use jelly::core::gdp::dependency::pom_donwloader::{MavenPomDownloader, PomDownloader};
use jelly::core::gdp::dependency::pom_managment::PomManagment;
use jelly::core::gdp::service::{dependency_service::DependencyService, pom_service::PomService};
use tokio;

#[tokio::main]
async fn main() {

    let managment = Pom;
    let downloader = MavenPomDownloader;

    match DependencyService::new(
        PomService::new(managment, downloader)
    )
    .start()
    .await {
        Ok(_) => print!("POM Processed Correclty."),
        Err(e) => eprint!("An error ocurred: {}", e),
    };

}
