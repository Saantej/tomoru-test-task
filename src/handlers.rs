use axum_client_ip::{InsecureClientIp, SecureClientIp};
use crate::state::IPS;

pub async fn handler(insecure_ip: InsecureClientIp, _secure_ip: SecureClientIp) {
    let mut map = IPS.lock().unwrap();
    *map.entry(insecure_ip.0.to_string()).or_insert(0) += 1;
}
