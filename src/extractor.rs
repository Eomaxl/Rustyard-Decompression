use std::fs;
use std::io;
use std::path::Path;

pub fn extract_files(filename: &str) -> Result<(), ()> {
    let fname = Path::new(filename);
    let file = fs::File::open(&fname).map_err(|_| ())?;
    let mut archive = zip::ZipArchive::new(file).map_err(|_| ())?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).map_err(|_| ())?;

        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };

        let comment = file.comment(); // This is already a &str
        if !comment.is_empty() {
            println!("File {} comment: {}", i, comment);
        }

        if file.name().ends_with('/') {
            println!("File {} extracted to \"{}\"", i, outpath.display());
            fs::create_dir_all(&outpath).map_err(|_| ())?;
        } else {
            println!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                outpath.display(),
                file.size()
            );
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(p).map_err(|_| ())?;
                }
            }
            let mut outfile = fs::File::create(&outpath).map_err(|_| ())?;
            io::copy(&mut file, &mut outfile).map_err(|_| ())?;
        }

        #[cfg(unix)]
        set_unix_permissions(&file, &outpath);
    }

    Ok(())
}

#[cfg(unix)]
fn set_unix_permissions(file: &zip::read::ZipFile, outpath: &Path) {
    use std::os::unix::fs::PermissionsExt;

    if let Some(mode) = file.unix_mode() {
        fs::set_permissions(outpath, fs::Permissions::from_mode(mode)).unwrap();
    }
}
