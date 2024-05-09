use async_vt3::VtClient;

#[tokio::main]
async fn main() {
    let mut args = std::env::args().skip(1);
    let api_key = match args
        .next()
        .ok_or("Please provide the api key as 1st argument!")
    {
        Ok(api_key) => api_key,
        Err(e) => {
            println!("{:?}", e);
            std::process::exit(1)
        }
    };

    let user_id = match args
        .next()
        .ok_or("Please provide the user id as 2nd argument!")
    {
        Ok(api_key) => api_key,
        Err(e) => {
            println!("{:?}", e);
            std::process::exit(1)
        }
    };

    let res = VtClient::new(&api_key).overall_quotas(&user_id).await;
    match res {
        Ok(report) => println!("{:#?}", report),
        Err(e) => println!("Error: {}", e),
    }
}
