#[cfg(not(target_env = "msvc"))]
use tikv_jemallocator::Jemalloc;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

#[tokio::main]
async fn main() {
    match vcl::init().await {
        Ok(_) => {}
        Err(_) => {
            tracing::error!("Can't launch server :(");
            return;
        }
    }

    vcl::core().await;
}
