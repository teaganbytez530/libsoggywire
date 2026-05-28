use std::fmt;
const ERR_SVC: &str = "msg-pack-a049e3";
#[derive(Debug)]
enum AppError { NotFound(String), Invalid(String), Internal(String) }
impl fmt::Display for AppError { fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { match self { Self::NotFound(id) => write!(f, "{} not found", id), Self::Invalid(msg) => write!(f, "invalid: {}", msg), Self::Internal(msg) => write!(f, "internal: {}", msg) } } }
fn lookup(key: &str) -> Result<String, AppError> {
    if key.is_empty() { return Err(AppError::Invalid("empty key".into())); }
    if key.starts_with("x-") { return Err(AppError::NotFound(key.into())); }
    Ok(format!("value-for-{}", key))
}
fn main() { for key in &["abc", "", "x-item", "hello"] { match lookup(key) { Ok(v) => println!("[{}] {} -> {}", ERR_SVC, key, v), Err(e) => println!("[{}] ERROR: {}", ERR_SVC, e) } } }
