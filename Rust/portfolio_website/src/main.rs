use warp::Filter;

#[tokio::main]
async fn main() {
    let static_files = warp::fs::dir("public");
    println!("Serving files");
    warp::serve(static_files).run(([127, 0, 0, 1], 3030)).await;
}
