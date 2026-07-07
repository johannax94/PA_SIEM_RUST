// ---- Règles existantes ----
pub mod failed_login;
pub mod password_spray;
pub mod account_compromise;
// pub mod bruteforce_ip; // ancienne signature (db, log), non branchée

// ---- Nouvelles règles ----
pub mod powershell_suspect;
pub mod cmd_execution;
pub mod rdp_foreign_country;
pub mod impossible_travel;
pub mod network_scan;
pub mod ransomware;
pub mod beaconing_c2;
pub mod dns_tunneling;
pub mod data_exfiltration;
pub mod privilege_escalation;
pub mod pass_the_hash;
pub mod rdp_bruteforce;
