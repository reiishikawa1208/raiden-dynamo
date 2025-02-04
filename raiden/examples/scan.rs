use raiden::*;

#[derive(Raiden, Debug)]
#[raiden(table_name = "ScanTestData0")]
#[allow(dead_code)]
pub struct ScanTestData0 {
    #[raiden(partition_key)]
    id: String,
    name: String,
    year: usize,
    num: usize,
}

#[tokio::main]
async fn main() {
    let client = ScanTestData0::client(Region::Custom {
        endpoint: "http://localhost:8000".into(),
        name: "ap-northeast-1".into(),
    });
    let res = client.scan().run().await;
    dbg!(&res);
}
