use warp::Filter;

#[tokio::main]
async fn main() {
	let root = warp::path::end()
		.map(|| "Hello, world!");
	
    warp::serve(root)
		.run(([0, 0, 0, 0], 8000))
		.await;
}
