use http_request::utils::post_request::post;
#[tokio::main]
/// This program is going to get the sensor reading from file of a directory and it will POST that
/// readings to the KAA server.
///
/// #Arguments
/// None
///
/// #Return
/// The program is going to print the body and the status of the Server.

async fn main() {
    let url="https://connect.cloud.kaaiot.com:443/kp1/c4edksqd4ks1slmo6mbg-v1/epmx/readings/update/keys";
    loop {
        let result = post(url).await;
        println!("Status {}", result.status());
        println!("Headers \n{:#?}", result.headers());
        println!("Body \n{}", result.text().await.unwrap());
    }
}
