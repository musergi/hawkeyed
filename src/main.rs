use hawkeye::generated::hawkeye_service_server::HawkeyeServiceServer;
use hawkeye::HawkeyeServiceImpl;
use systemstat::{Platform, System};
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let system = System::new();
    let service = HawkeyeServiceImpl::new(system);
    let addr = "[::1]:50051".parse()?;
    Server::builder()
        .add_service(HawkeyeServiceServer::new(service))
        .serve(addr)
        .await?;
    Ok(())
}
