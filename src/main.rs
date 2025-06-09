use kuksa_rust_sdk::kuksa::val::v2::KuksaClientV2;
use kuksa_rust_sdk::kuksa::common::ClientTraitV2;

#[tokio::main]
async fn main() {
    let host = "http://localhost:55555";
    let mut client = KuksaClientV2::from_host(host);
        match client.get_value("Vehicle.Speed".to_owned()).await {
        Ok(response) => {
            println!("Got value for Vehicle.Speed: {:?}", response);
        }
        Err(err) => {
            println!(
                "Getting value for signal {:?} failed: {:?}",
                "Vehicle.Speed", err
            );
        }
    }
}
