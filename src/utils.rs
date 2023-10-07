pub fn format_bytes(mut bytes: usize) -> String {
    const UNITS: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];

    let mut unit = 0;
    while bytes >= 1024 && unit < UNITS.len() - 1 {
        bytes /= 1024;
        unit += 1;
    }

    format!("{}{}", bytes, UNITS[unit])
}
