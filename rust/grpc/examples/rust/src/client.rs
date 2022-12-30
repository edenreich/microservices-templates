pub mod pb {
    pub mod payments {
        include!("../codegen/payments.rs");
    }
}

use pb::payments::{BtcPaymentRequest, bitcoin_client::BitcoinClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = BitcoinClient::connect("http://0.0.0.0:50051").await?;

    let request = tonic::Request::new(BtcPaymentRequest {
        from_address: "mwSmXgUGd78mUup5332qCyDUy32LoGCAfJ".to_string(),
        to_address: "myPrkbuYHZNGB8CPnZbDNzp945SLcjPK6z".to_string(),
        amount: 22,
    });

    let response = client.send_payment(request).await?;

    println!("Response from gRPC server={:?}", response);

    Ok(())
}
