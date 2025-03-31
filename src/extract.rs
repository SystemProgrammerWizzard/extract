pub fn extract_tar_gz(input_path: &str) -> Result<(), std::io::Error> {
    let tar_gz = std::fs::File::open(input_path)?;
    let tar = flate2::read::GzDecoder::new(tar_gz);
    let mut archive = tar::Archive::new(tar);
    archive.unpack(".")?;
    Ok(())
}



pub fn extract_archive_zip(path: &str) -> Result<(), std::io::Error> {
    let file = std::fs::File::open(path)?;
    let mut archive = zip::read::ZipArchive::new(file)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = file.sanitized_name();
        
        if file.name().ends_with('/') {
            std::fs::create_dir_all(&outpath)?;
        } else {
            if let Some(parent) = outpath.parent() {
                std::fs::create_dir_all(parent)?;
            }
            let mut outfile = std::fs::File::create(&outpath)?;
            std::io::copy(&mut file, &mut outfile)?;
        }
    }

    Ok(())
}

