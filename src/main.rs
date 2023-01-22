use clap::Parser;
use hawkeye::generated::hawkeye_service_server::HawkeyeServiceServer;
use hawkeye::HawkeyeServiceImpl;
use systemstat::{Platform, System};
use tonic::transport::Server;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    address: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let system = System::new();
    let service = HawkeyeServiceImpl::new(system);
    let addr = args.address.parse()?;
    Server::builder()
        .add_service(HawkeyeServiceServer::new(service))
        .serve(addr)
        .await?;
    Ok(())
}
