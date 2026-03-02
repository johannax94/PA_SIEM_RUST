use crate::models::Log;

pub fn check_bruteforce(logs: &Vec<Log>) -> Option<String> {
    let failed_count = logs
        .iter()
        .filter(|log| log.message == "login failed")
        .count();

    if failed_count >= 3 {
        Some("Tentative de bruteforce détectée !".to_string())
    } else {
        None
    }
}