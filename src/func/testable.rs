use super::not_testable::local_ip_address_formated;
use std::{fs, path::Path};

pub fn ensure_output_dir(path: &str) -> std::io::Result<()> {
    if !Path::new(path).is_dir() {
        fs::create_dir_all(path)?;
    }
    return Ok(());
}

pub fn remove_old_files(path: &str) -> std::io::Result<()> {
    if Path::new(path).is_file() {
        return Ok(());
    }
    let my_local_ip_addr = local_ip_address_formated();
    for child in fs::read_dir(path)? {
        let child = child.unwrap();
        let path = child.path();
        if path.is_dir() {
            continue;
        }
        let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
        if file_name.starts_with(&my_local_ip_addr) {
            fs::remove_file(path).unwrap();
        }
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use std::{fs, path::Path};

    use super::*;

    #[test]
    fn test_ensure_output_dir() {
        let path = Path::new("./test");
        assert_eq!(path.is_dir(), false);
        match ensure_output_dir("./test") {
            Ok(_) => (),
            Err(e) => println!("Error {:?}", e),
        }
        assert_eq!(path.is_dir(), true);
        match ensure_output_dir("./test") {
            Ok(_) => (),
            Err(e) => println!("Error {:?}", e),
        }
        assert_eq!(path.is_dir(), true);
        fs::remove_dir_all("./test").unwrap();
    }

    #[test]
    fn test_remove_old_files() {
        let ip_address_formated: String = local_ip_address_formated();
        fs::create_dir_all("./tests").unwrap();
        fs::File::create(format!("./tests/{}_1234_1.txt", ip_address_formated)).unwrap();
        fs::File::create(format!("./tests/{}_1235_2.txt", ip_address_formated)).unwrap();
        remove_old_files("./tests").unwrap();
        assert!(fs::read_dir("./tests").unwrap().collect::<Vec<_>>().len() == 0);
        fs::remove_dir_all("./tests").unwrap();
    }
}
