use std::{ffi::OsStr, fs::DirEntry, path::PathBuf, str::FromStr};

fn should_delete(file_name: &OsStr) -> bool {
    file_name == "node_modules"
}

fn delete_item(dir_entry: &DirEntry) -> Result<(), std::io::Error> {
    match dir_entry.metadata().map(|data| data.is_dir()) {
        Ok(true) => std::fs::remove_dir_all(dir_entry.path()),
        Ok(false) => std::fs::remove_file(dir_entry.path()),
        Err(err) => {
            eprintln!("Error deleting item: {:?}", err);
            Err(err)
        }
    }
}

fn check_dir(path: PathBuf) {
    if let Ok(current_dir) = std::fs::read_dir(path) {
        for i in current_dir {
            if let Ok(index) = i {
                if should_delete(&index.file_name()) {
                    if let Ok(_) = delete_item(&index) {
                        println!("deleted file: {:?}", index.file_name());
                    }
                } else if index
                    .metadata()
                    .map(|data| data.is_dir())
                    .unwrap_or_default()
                {
                    check_dir(index.path())
                }
            }
        }
    }
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let path = PathBuf::from_str(args.get(1).unwrap()).unwrap();

    check_dir(path);
}
