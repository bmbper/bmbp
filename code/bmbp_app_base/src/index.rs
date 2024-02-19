use salvo::handler;

#[handler]
pub async fn index() -> String {
    "欢迎访问".to_string()
}