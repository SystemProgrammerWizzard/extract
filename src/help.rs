


pub fn convert_relative_path_to_absolute_path(path: &str) -> std::io::Result<String> {
    let path = std::fs::canonicalize(path)?;
    Ok(path.to_str().unwrap().to_string())
}
