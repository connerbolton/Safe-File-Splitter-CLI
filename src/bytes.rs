/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use anyhow::Result;

/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn size_to_bytes(size: &String) -> Result<u64> {

    let size = size.to_uppercase();

    let byte_multiplyer: u64 = 
        match true {
            _ if size.ends_with("GB") => 1073741824,
            _ if size.ends_with("MB") => 1048576,
            _ if size.ends_with("KB") => 1024,
            _ => 1,
        };

    let mut byte_size: u64 = size
        .trim_end_matches(|c: char| c.is_alphabetic())
        .parse()
        .map_err(|_| anyhow::anyhow!("Invalid size '{}', expected something like 1GB, 500MB, 100KB", size))?;
    
    byte_size = byte_size * byte_multiplyer;

    return Ok(byte_size);

}

/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn format_bytes(bytes: u64) -> String {
    if bytes >= 1073741824 {
        format!("{:.2} GB", bytes as f64 / 1073741824 as f64)
    } else if bytes >= 1048576 {
        format!("{:.2} MB", bytes as f64 / 1048576 as f64)
    } else if bytes >= 1024 {
        format!("{:.2} KB", bytes as f64 / 1024 as f64)
    } else {
        format!("{} B", bytes)
    }
}

/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////