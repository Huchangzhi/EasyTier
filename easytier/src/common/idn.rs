use idna::domain_to_ascii;

/// Convert an internationalized domain name to ASCII (PunyCode) format.
/// 
/// This function processes a URL string and converts any internationalized 
/// domain names to their ASCII (PunyCode) equivalents, which is required 
/// for proper handling of non-ASCII domains in network protocols.
pub fn convert_idn_to_ascii(url_str: &str) -> Result<String, String> {
    // Check if the URL string contains non-ASCII characters
    if url_str.chars().any(|c| !c.is_ascii()) {
        // Extract the scheme part
        let mut url_parts = url_str.splitn(2, "://");
        let scheme = url_parts.next().unwrap_or("");
        let rest = url_parts.next().unwrap_or(url_str);
        
        // Extract host and port part, path part
        let mut path_parts = rest.splitn(2, '/');
        let host_port_part = path_parts.next().unwrap_or("");
        let path_part = path_parts.next().map(|s| format!("/{}", s)).unwrap_or_default();
        
        // Extract port if exists
        let (host_part, port_part) = if let Some(pos) = host_port_part.rfind(':') {
            // Check if it's a port number (not IPv6 format)
            let port_str = &host_port_part[pos+1..];
            if port_str.chars().all(|c| c.is_ascii_digit()) {
                (&host_port_part[..pos], format!(":{}", port_str))
            } else {
                // It might be IPv6 or just part of hostname, treat as hostname
                (host_port_part, String::new())
            }
        } else {
            (host_port_part, String::new())
        };
        
        // Check if the host part contains non-ASCII characters
        if host_part.chars().any(|c| !c.is_ascii()) {
            // If host contains non-ASCII chars, convert to PunyCode
            let ascii_host = domain_to_ascii(host_part)
                .map_err(|e| format!("Failed to convert IDN to ASCII: {}", e))?;
            
            // Reconstruct the URL
            let result = format!("{}://{}{}{}", scheme, ascii_host, port_part, path_part);
            Ok(result)
        } else {
            // Host part is already ASCII, return original
            Ok(url_str.to_string())
        }
    } else {
        // Entire URL has ASCII characters, no conversion needed
        Ok(url_str.to_string())
    }
}

/// A safe version of IDN conversion that falls back to the original string if conversion fails
pub fn safe_convert_idn_to_ascii(url_str: &str) -> String {
    convert_idn_to_ascii(url_str).unwrap_or_else(|_| url_str.to_string())
}