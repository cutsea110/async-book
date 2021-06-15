use std::io;

async fn hello_world() {
    println!("hello, world!");
}

#[async_std::main]
async fn main() -> Result<(), io::Error> {
    Ok(hello_world().await)
}
