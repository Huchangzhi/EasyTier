use idna::{domain_to_ascii, domain_to_unicode};

/// Convert an internationalized domain name to ASCII (PunyCode) format.
/// 
/// This function processes a URL string and converts any internationalized 
/// domain names to their ASCII (PunyCode) equivalents, which is required 
/// for proper handling of non-ASCII domains in network protocols.
pub fn convert_idn_to_ascii(url_str: &str) -> Result<String, String> {
    let parsed_url = url::Url::parse(url_str).map_err(|e| e.to_string())?;
    
    let scheme = parsed_url.scheme();
    let port = parsed_url.port();
    let path_and_rest = {
        let mut path = parsed_url.path().to_string();
        if let Some(query) = parsed_url.query() {
            path.push_str(&format!("?{}", query));
        }
        if let Some(fragment) = parsed_url.fragment() {
            path.push_str(&format!("#{}", fragment));
        }
        path
    };
    
    // Get the host, convert it if it contains non-ASCII characters
    if let Some(host_str) = parsed_url.host_str() {
        // Check if the host contains non-ASCII characters
        if host_str.chars().any(|c| !c.is_ascii()) {
            // This host has non-ASCII characters, convert to ASCII/PunyCode
            let ascii_host = domain_to_ascii(host_str)
                .map_err(|e| format!("Failed to convert IDN to ASCII: {}", e))?;
            
            // Reconstruct the URL with the ASCII host
            let mut result = format!("{}://{}", scheme, ascii_host);
            if let Some(port) = port {
                result.push_str(&format!(":{}", port));
            }
            result.push_str(&path_and_rest);
            
            Ok(result)
        } else {
            // Host is already ASCII, return original
            Ok(url_str.to_string())
        }
    } else {
        // No host (e.g., relative URL), return original
        Ok(url_str.to_string())
    }
}

/// A safe version of IDN conversion that falls back to the original string if conversion fails
pub fn safe_convert_idn_to_ascii(url_str: &str) -> String {
    convert_idn_to_ascii(url_str).unwrap_or_else(|_| url_str.to_string())
}