use rust_call_example::logic::get_principals;

#[tokio::main]
async fn main() {
    let principals = get_principals().await;
    println!("{:?}", principals);
}