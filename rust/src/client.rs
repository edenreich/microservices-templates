use payments::bitcoin_client::BitcoinClient;
use payments::BtcPaymentRequest;

pub mod payments {
    tonic::include_proto!("payments");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = BitcoinClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(BtcPaymentRequest {
        from_address: "1F1tAaz".to_string(),
        to_address: "1F1tAaz".to_string(),
        amount: 22,
    });

    let response = client.send_payment(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
