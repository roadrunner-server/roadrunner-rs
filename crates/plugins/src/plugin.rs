/// A trait that defines the interface for a plugin.
pub trait Plugin {
    async fn init(cfg: serde_json::Value) -> anyhow::Result<()>;
    async fn serve() -> anyhow::Result<()>;
    async fn stop() -> anyhow::Result<()>;
    async fn restart() -> anyhow::Result<()>;
}
