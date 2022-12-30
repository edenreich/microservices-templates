
use microservice_example::core::{register_signals, run_grpc};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    register_signals()?;

    loop {
        tokio::select! {
            _ = run_grpc() => println!("gRPC server terminated!"),
        }
        println!("Restarting...");
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}
