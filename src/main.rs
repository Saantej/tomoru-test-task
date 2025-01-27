mod state;
mod handlers;
mod routes;

use crate::state::IPS;
use std::net::SocketAddr;
use tokio::time::{interval, Duration};
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    tokio::spawn(async move  {
        let mut ticker = interval(Duration::from_secs(1));
        loop {
            ticker.tick().await;

            let guard = IPS.lock().unwrap();
            let cloned: HashMap<String, i32> = guard.clone();
            drop(guard);

            let mut entries: Vec<(String, i32)> = cloned.into_iter().collect();

            entries.sort_by(|(_, k), (_, k2)| k2.cmp(k));
            println!("IPS: ");
            println!("{:?}", entries);
        }
    });

    let app = routes::create_router();

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}
