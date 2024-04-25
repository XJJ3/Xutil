// 定义一个命令 trait，所有命令都会实现这个 trait
pub trait CommandTrait: Sized + Send + Sync + 'static {
    fn execute(args: &serde_json::Value) -> Result<serde_json::Value, String>;
}
pub trait AsyncCommandTrait: Sized + Send + Sync + 'static {
    async fn execute(args: &serde_json::Value) -> Result<serde_json::Value, String>;
}
