// When executing the below code nothing is printed to console. Can you guess what is missing?

#[tokio::main]
async fn main() {
    my_function().await;
}

async fn my_function() {
    println!("My first asynchronous function in rust!");
}
