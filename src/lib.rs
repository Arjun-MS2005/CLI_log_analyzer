use::std::collections::HashMap;

pub fn analyze_logs<'a>(contents: &'a str) -> HashMap<&'a str, u32> {
    let mut log_hash: HashMap<&'a str, u32> = HashMap::new();
    const LOG_LEVELS: &[&str] = &["INFO", "WARN", "ERROR", "DEBUG"];

    for line in contents.lines() {
        for level in LOG_LEVELS {
            if line.contains(&format!("[{}]", level)) {
                log_hash.entry(level)
                        .and_modify(|count| *count += 1)
                        .or_insert(1);
                break;
            }
        }
    }
    log_hash
}
