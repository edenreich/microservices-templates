use tonic::{transport::Server};

pub mod pb {
    pub mod payments {
        include!("../codegen/payments.rs");
    }
}

use tonic::{Request, Response, Status};
use pb::payments::bitcoin_server::BitcoinServer;
use pb::payments::bitcoin_server::{Bitcoin};
use pb::payments::{BtcPaymentRequest, BtcPaymentResponse};

pub mod handlers {
    #[derive(Debug, Default)]
    pub struct BitcoinService {}

    #[tonic::async_trait]
    impl crate::Bitcoin for BitcoinService {
        async fn send_payment(
            &self,
            request: crate::Request<crate::BtcPaymentRequest>,
        ) -> Result<crate::Response<crate::BtcPaymentResponse>, crate::Status> {
            let request = request.into_inner();
            println!("Received request: {:?}", request);
            let reply = crate::BtcPaymentResponse {
                successful: true,
                message: format!("Sent {} BTC to {}", request.amount, request.to_address).into(),
            };

            Ok(crate::Response::new(reply))
        }
    }
}

pub mod core {
    use crate::handlers::{BitcoinService};

    pub fn register_signals() -> Result<(), Box<dyn std::error::Error>> {
        let ctrl_c = tokio::signal::ctrl_c();
        tokio::spawn(async move {
            ctrl_c.await.unwrap();
            println!("Ctrl+C received, shutting down");
            std::process::exit(0);
        });
    
        let mut sigterm = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())?;
        tokio::spawn(async move {
            sigterm.recv().await;
            println!("SIGTERM received, shutting down");
            std::process::exit(0);
        });

        Ok(())
    }

    pub async fn run_grpc() {
        println!("gRPC server is listening on ::50051");
        let bitcoin_service = BitcoinService::default();
        let addr = "0.0.0.0:50051".parse().unwrap();
        crate::Server::builder()
            .add_service(crate::BitcoinServer::new(bitcoin_service))
            .serve(addr)
            .await
            .unwrap()
    }
}
