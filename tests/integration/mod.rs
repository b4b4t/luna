use dotenv::dotenv;
use luna_core::{self, core::service::sqlserver_provider::get_client};

#[tokio::test]
async fn it_works() {
    // .env variables
    dotenv().ok();

    let query = "";
    let client = get_client().await;
    println!("query : {}", query);
    // println!("Execute query : {}", query);
    let mut binding = client.expect("Client error");
    let rows = binding.query(query, &[&1i32]).await;
    println!("{:?}", rows);
    rows.unwrap(); //.into_first_result().await.unwrap();
}
