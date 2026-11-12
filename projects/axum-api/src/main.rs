use axum_api::{AppState, app};
use tokio::{net::TcpListener, signal};

async fn shutdown_signal() {
    signal::ctrl_c()
        .await
        .expect("نصب handler برای Ctrl+C شکست خورد");
    tracing::info!("سیگنال خاموشی دریافت شد؛ درخواست‌های در حال انجام تمام می‌شوند");
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    tracing::info!("گوش دادن روی {}", listener.local_addr().unwrap());

    axum::serve(listener, app(AppState::default()))
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}
