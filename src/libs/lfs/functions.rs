use super::{super::error, Scopes, Value, LIB_NAME};
use std::fs::{self, read, write};
use std::path::PathBuf;

const VALUE_TYPE: &str = "func";

/// Read a file and return its contents as an array of bytes
pub fn lread(args: Vec<Value>, _scopes: &mut Scopes) -> Value {
    const FUNC_NAME: &str = "read";

    if args.len() != 1 {
        return Value::Error(
            "<func> read: Incorrect count of arguments! Expected 1 args".to_string(),
        );
    }

    match &args[0] {
        Value::String(path) => {
            if PathBuf::from(path).exists() {
                match read(path) {
                    Ok(data) => Value::Array(data.iter().map(|&x| Value::Byte(x)).collect()),
                    Err(err_text) => Value::Error(error::fmt(
                        LIB_NAME,
                        VALUE_TYPE,
                        FUNC_NAME,
                        "FileReadError",
                        &err_text.to_string(),
                    )),
                }
            } else {
                Value::Error(error::fmt(
                    LIB_NAME,
                    VALUE_TYPE,
                    FUNC_NAME,
                    "FileNotFoundError",
                    &format!("file {} not found!", path),
                ))
            }
        }
        Value::Error(err_text) => Value::Error(format!(
            "{}\n{}",
            error::fmt(
                LIB_NAME,
                VALUE_TYPE,
                FUNC_NAME,
                "CatchedError",
                "caught error",
            ),
            err_text
        )),
        _ => Value::Error(error::fmt(
            LIB_NAME,
            VALUE_TYPE,
            FUNC_NAME,
            "TypeError",
            "expected String type in arguments",
        )),
    }
}

/// Write data to a file
pub fn lwrite(args: Vec<Value>, _scopes: &mut Scopes) -> Value {
    const FUNC_NAME: &str = "write";

    if args.len() != 2 {
        return Value::Error(format!(
            "<func> {}: Incorrect count of arguments! Expected 2 args",
            FUNC_NAME
        ));
    }

    match (&args[0], &args[1]) {
        (Value::String(path), Value::Array(array)) => {
            let bytes: Vec<u8> = array
                .iter()
                .filter_map(|val| {
                    if let Value::Byte(b) = val {
                        Some(*b)
                    } else {
                        None
                    }
                })
                .collect();

            match write(path, bytes) {
                Ok(_) => Value::Nil,
                Err(err_text) => Value::Error(error::fmt(
                    LIB_NAME,
                    VALUE_TYPE,
                    FUNC_NAME,
                    "FileWriteError",
                    &err_text.to_string(),
                )),
            }
        }
        (Value::Error(err_text), _) | (_, Value::Error(err_text)) => Value::Error(format!(
            "{}\n{}",
            error::fmt(
                LIB_NAME,
                VALUE_TYPE,
                FUNC_NAME,
                "CatchedError",
                "caught error",
            ),
            err_text
        )),
        _ => Value::Error(error::fmt(
            LIB_NAME,
            VALUE_TYPE,
            FUNC_NAME,
            "TypeError",
            "expected String type for path and Array type for data",
        )),
    }
}

/// List contents of a directory
pub fn llistdir(args: Vec<Value>, _scopes: &mut Scopes) -> Value {
    const FUNC_NAME: &str = "ls";

    if args.len() != 1 {
        return Value::Error(format!(
            "<func> {}: Incorrect count of arguments! Expected 1 arg",
            FUNC_NAME
        ));
    }

    match &args[0] {
        Value::String(path) => {
            let path_buf = PathBuf::from(path);
            if !path_buf.exists() {
                return Value::Error(error::fmt(
                    LIB_NAME,
                    VALUE_TYPE,
                    FUNC_NAME,
                    "DirectoryNotFoundError",
                    &format!("directory {} not found!", path),
                ));
            }

            if !path_buf.is_dir() {
                return Value::Error(error::fmt(
                    LIB_NAME,
                    VALUE_TYPE,
                    FUNC_NAME,
                    "NotADirectoryError",
                    &format!("{} is not a directory!", path),
                ));
            }

            match fs::read_dir(&path_buf) {
                Ok(entries) => {
                    let mut result = Vec::new();
                    for entry in entries {
                        match entry {
                            Ok(entry) => {
                                if let Ok(entry_name) = entry.file_name().into_string() {
                                    result.push(Value::String(entry_name));
                                }
                            }
                            Err(_) => continue, // Skip entries we can't read
                        }
                    }
                    Value::Array(result)
                }
                Err(err_text) => Value::Error(error::fmt(
                    LIB_NAME,
                    VALUE_TYPE,
                    FUNC_NAME,
                    "DirectoryReadError",
                    &err_text.to_string(),
                )),
            }
        }
        Value::Error(err_text) => Value::Error(format!(
            "{}\n{}",
            error::fmt(
                LIB_NAME,
                VALUE_TYPE,
                FUNC_NAME,
                "CatchedError",
                "caught error",
            ),
            err_text
        )),
        _ => Value::Error(error::fmt(
            LIB_NAME,
            VALUE_TYPE,
            FUNC_NAME,
            "TypeError",
            "expected String type for directory path",
        )),
    }
}

/// Check if a path exists in the filesystem
pub fn lexists(args: Vec<Value>, _scopes: &mut Scopes) -> Value {
    const FUNC_NAME: &str = "exists";

    if args.len() != 1 {
        return Value::Error(format!(
            "<func> {}: Incorrect count of arguments! Expected 1 arg",
            FUNC_NAME
        ));
    }

    match &args[0] {
        Value::String(path) => Value::Bool(PathBuf::from(path).exists()),
        Value::Error(err_text) => Value::Error(format!(
            "{}\n{}",
            error::fmt(
                LIB_NAME,
                VALUE_TYPE,
                FUNC_NAME,
                "CatchedError",
                "caught error",
            ),
            err_text
        )),
        _ => Value::Error(error::fmt(
            LIB_NAME,
            VALUE_TYPE,
            FUNC_NAME,
            "TypeError",
            "expected String type for path",
        )),
    }
}

/// Create a directory
pub fn lmkdir(args: Vec<Value>, _scopes: &mut Scopes) -> Value {
    const FUNC_NAME: &str = "mkdir";

    if args.len() != 1 && args.len() != 2 {
        return Value::Error(format!(
            "<func> {}: Incorrect count of arguments! Expected 1 or 2 args",
            FUNC_NAME
        ));
    }

    let create_parents = if args.len() == 2 {
        match &args[1] {
            Value::Bool(recursive) => *recursive,
            _ => false,
        }
    } else {
        false
    };

    match &args[0] {
        Value::String(path) => {
            let result = if create_parents {
                fs::create_dir_all(path)
            } else {
                fs::create_dir(path)
            };

            match result {
                Ok(_) => Value::Nil,
                Err(err_text) => Value::Error(error::fmt(
                    LIB_NAME,
                    VALUE_TYPE,
                    FUNC_NAME,
                    "DirectoryCreateError",
                    &err_text.to_string(),
                )),
            }
        }
        Value::Error(err_text) => Value::Error(format!(
            "{}\n{}",
            error::fmt(
                LIB_NAME,
                VALUE_TYPE,
                FUNC_NAME,
                "CatchedError",
                "caught error",
            ),
            err_text
        )),
        _ => Value::Error(error::fmt(
            LIB_NAME,
            VALUE_TYPE,
            FUNC_NAME,
            "TypeError",
            "expected String type for directory path",
        )),
    }
}

/// Remove a file
pub fn lrm(args: Vec<Value>, _scopes: &mut Scopes) -> Value {
    const FUNC_NAME: &str = "rm";

    if args.len() != 1 {
        return Value::Error(format!(
            "<func> {}: Incorrect count of arguments! Expected 1 arg",
            FUNC_NAME
        ));
    }

    match &args[0] {
        Value::String(path) => {
            let path_buf = PathBuf::from(path);
            if !path_buf.exists() {
                return Value::Error(error::fmt(
                    LIB_NAME,
                    VALUE_TYPE,
                    FUNC_NAME,
                    "FileNotFoundError",
                    &format!("file {} not found!", path),
                ));
            }

            if path_buf.is_dir() {
                return Value::Error(error::fmt(
                    LIB_NAME,
                    VALUE_TYPE,
                    FUNC_NAME,
                    "IsDirectoryError",
                    &format!("{} is a directory! Use rmdir instead.", path),
                ));
            }

            match fs::remove_file(path) {
                Ok(_) => Value::Nil,
                Err(err_text) => Value::Error(error::fmt(
                    LIB_NAME,
                    VALUE_TYPE,
                    FUNC_NAME,
                    "FileRemoveError",
                    &err_text.to_string(),
                )),
            }
        }
        Value::Error(err_text) => Value::Error(format!(
            "{}\n{}",
            error::fmt(
                LIB_NAME,
                VALUE_TYPE,
                FUNC_NAME,
                "CatchedError",
                "caught error",
            ),
            err_text
        )),
        _ => Value::Error(error::fmt(
            LIB_NAME,
            VALUE_TYPE,
            FUNC_NAME,
            "TypeError",
            "expected String type for file path",
        )),
    }
}

/// Remove a directory
pub fn lrmdir(args: Vec<Value>, _scopes: &mut Scopes) -> Value {
    const FUNC_NAME: &str = "rmdir";

    if args.len() != 1 && args.len() != 2 {
        return Value::Error(format!(
            "<func> {}: Incorrect count of arguments! Expected 1 or 2 args",
            FUNC_NAME
        ));
    }

    let recursive = if args.len() == 2 {
        match &args[1] {
            Value::Bool(recursive) => *recursive,
            _ => false,
        }
    } else {
        false
    };

    match &args[0] {
        Value::String(path) => {
            let path_buf = PathBuf::from(path);
            if !path_buf.exists() {
                return Value::Error(error::fmt(
                    LIB_NAME,
                    VALUE_TYPE,
                    FUNC_NAME,
                    "DirectoryNotFoundError",
                    &format!("directory {} not found!", path),
                ));
            }

            if !path_buf.is_dir() {
                return Value::Error(error::fmt(
                    LIB_NAME,
                    VALUE_TYPE,
                    FUNC_NAME,
                    "NotADirectoryError",
                    &format!("{} is not a directory!", path),
                ));
            }

            let result = if recursive {
                fs::remove_dir_all(path)
            } else {
                fs::remove_dir(path)
            };

            match result {
                Ok(_) => Value::Nil,
                Err(err_text) => Value::Error(error::fmt(
                    LIB_NAME,
                    VALUE_TYPE,
                    FUNC_NAME,
                    "DirectoryRemoveError",
                    &err_text.to_string(),
                )),
            }
        }
        Value::Error(err_text) => Value::Error(format!(
            "{}\n{}",
            error::fmt(
                LIB_NAME,
                VALUE_TYPE,
                FUNC_NAME,
                "CatchedError",
                "caught error",
            ),
            err_text
        )),
        _ => Value::Error(error::fmt(
            LIB_NAME,
            VALUE_TYPE,
            FUNC_NAME,
            "TypeError",
            "expected String type for directory path",
        )),
    }
}

/// Rename/move a file or directory
pub fn lmv(args: Vec<Value>, _scopes: &mut Scopes) -> Value {
    const FUNC_NAME: &str = "mv";

    if args.len() != 2 {
        return Value::Error(format!(
            "<func> {}: Incorrect count of arguments! Expected 2 args",
            FUNC_NAME
        ));
    }

    match (&args[0], &args[1]) {
        (Value::String(from), Value::String(to)) => {
            let from_path = PathBuf::from(from);

            if !from_path.exists() {
                return Value::Error(error::fmt(
                    LIB_NAME,
                    VALUE_TYPE,
                    FUNC_NAME,
                    "SourceNotFoundError",
                    &format!("source path {} not found!", from),
                ));
            }

            match fs::rename(from, to) {
                Ok(_) => Value::Nil,
                Err(err_text) => Value::Error(error::fmt(
                    LIB_NAME,
                    VALUE_TYPE,
                    FUNC_NAME,
                    "RenameError",
                    &err_text.to_string(),
                )),
            }
        }
        (Value::Error(err_text), _) | (_, Value::Error(err_text)) => Value::Error(format!(
            "{}\n{}",
            error::fmt(
                LIB_NAME,
                VALUE_TYPE,
                FUNC_NAME,
                "CatchedError",
                "caught error",
            ),
            err_text
        )),
        _ => Value::Error(error::fmt(
            LIB_NAME,
            VALUE_TYPE,
            FUNC_NAME,
            "TypeError",
            "expected String types for source and destination paths",
        )),
    }
}

/// Copy a file
pub fn lcp(args: Vec<Value>, _scopes: &mut Scopes) -> Value {
    const FUNC_NAME: &str = "cp";

    if args.len() != 2 {
        return Value::Error(format!(
            "<func> {}: Incorrect count of arguments! Expected 2 args",
            FUNC_NAME
        ));
    }

    match (&args[0], &args[1]) {
        (Value::String(from), Value::String(to)) => {
            let from_path = PathBuf::from(from);

            if !from_path.exists() {
                return Value::Error(error::fmt(
                    LIB_NAME,
                    VALUE_TYPE,
                    FUNC_NAME,
                    "SourceNotFoundError",
                    &format!("source path {} not found!", from),
                ));
            }

            if from_path.is_dir() {
                return Value::Error(error::fmt(
                    LIB_NAME,
                    VALUE_TYPE,
                    FUNC_NAME,
                    "IsDirectoryError",
                    &format!("{} is a directory! Use recursive copy instead.", from),
                ));
            }

            match fs::copy(from, to) {
                Ok(_) => Value::Nil,
                Err(err_text) => Value::Error(error::fmt(
                    LIB_NAME,
                    VALUE_TYPE,
                    FUNC_NAME,
                    "CopyError",
                    &err_text.to_string(),
                )),
            }
        }
        (Value::Error(err_text), _) | (_, Value::Error(err_text)) => Value::Error(format!(
            "{}\n{}",
            error::fmt(
                LIB_NAME,
                VALUE_TYPE,
                FUNC_NAME,
                "CatchedError",
                "caught error",
            ),
            err_text
        )),
        _ => Value::Error(error::fmt(
            LIB_NAME,
            VALUE_TYPE,
            FUNC_NAME,
            "TypeError",
            "expected String types for source and destination paths",
        )),
    }
}

/// Get the current working directory
pub fn lpwd(args: Vec<Value>, _scopes: &mut Scopes) -> Value {
    const FUNC_NAME: &str = "pwd";

    if !args.is_empty() {
        return Value::Error(format!(
            "<func> {}: Incorrect count of arguments! Expected 0 args",
            FUNC_NAME
        ));
    }

    match std::env::current_dir() {
        Ok(path) => match path.to_str() {
            Some(path_str) => Value::String(path_str.to_string()),
            None => Value::Error(error::fmt(
                LIB_NAME,
                VALUE_TYPE,
                FUNC_NAME,
                "PathConversionError",
                "Failed to convert path to string",
            )),
        },
        Err(err_text) => Value::Error(error::fmt(
            LIB_NAME,
            VALUE_TYPE,
            FUNC_NAME,
            "CurrentDirError",
            &err_text.to_string(),
        )),
    }
}

/// Get file or directory info
pub fn lstat(args: Vec<Value>, _scopes: &mut Scopes) -> Value {
    const FUNC_NAME: &str = "stat";

    if args.len() != 1 {
        return Value::Error(format!(
            "<func> {}: Incorrect count of arguments! Expected 1 arg",
            FUNC_NAME
        ));
    }

    match &args[0] {
        Value::String(path) => {
            let path_buf = PathBuf::from(path);

            if !path_buf.exists() {
                return Value::Error(error::fmt(
                    LIB_NAME,
                    VALUE_TYPE,
                    FUNC_NAME,
                    "PathNotFoundError",
                    &format!("path {} not found!", path),
                ));
            }

            let mut info = Vec::new();

            // Check if it's a file or directory
            info.push((
                "type".to_string(),
                if path_buf.is_dir() {
                    Value::String("directory".to_string())
                } else if path_buf.is_file() {
                    Value::String("file".to_string())
                } else if path_buf.is_symlink() {
                    Value::String("symlink".to_string())
                } else {
                    Value::String("unknown".to_string())
                },
            ));

            // Get file size if it's a file
            if path_buf.is_file() {
                match fs::metadata(path) {
                    Ok(metadata) => {
                        info.push(("size".to_string(), Value::Number(metadata.len() as i128)));

                        // Try to get modification time
                        if let Ok(mtime) = metadata.modified() {
                            if let Ok(duration) = mtime.duration_since(std::time::UNIX_EPOCH) {
                                info.push((
                                    "modified".to_string(),
                                    Value::Number(duration.as_secs() as i128),
                                ));
                            }
                        }

                        // Try to get creation time
                        if let Ok(ctime) = metadata.created() {
                            if let Ok(duration) = ctime.duration_since(std::time::UNIX_EPOCH) {
                                info.push((
                                    "created".to_string(),
                                    Value::Number(duration.as_secs() as i128),
                                ));
                            }
                        }

                        // Try to get access time
                        if let Ok(atime) = metadata.accessed() {
                            if let Ok(duration) = atime.duration_since(std::time::UNIX_EPOCH) {
                                info.push((
                                    "accessed".to_string(),
                                    Value::Number(duration.as_secs() as i128),
                                ));
                            }
                        }
                    }
                    Err(err_text) => {
                        return Value::Error(error::fmt(
                            LIB_NAME,
                            VALUE_TYPE,
                            FUNC_NAME,
                            "MetadataError",
                            &err_text.to_string(),
                        ));
                    }
                }
            }

            // Convert the info to a Lisp struct (represented as association list)
            Value::Array(
                info.into_iter()
                    .map(|(k, v)| Value::Array(vec![Value::String(k), v]))
                    .collect(),
            )
        }
        Value::Error(err_text) => Value::Error(format!(
            "{}\n{}",
            error::fmt(
                LIB_NAME,
                VALUE_TYPE,
                FUNC_NAME,
                "CatchedError",
                "caught error",
            ),
            err_text
        )),
        _ => Value::Error(error::fmt(
            LIB_NAME,
            VALUE_TYPE,
            FUNC_NAME,
            "TypeError",
            "expected String type for path",
        )),
    }
}

/// Create a symbolic link
pub fn lsymlink(args: Vec<Value>, _scopes: &mut Scopes) -> Value {
    const FUNC_NAME: &str = "symlink";

    if args.len() != 2 {
        return Value::Error(format!(
            "<func> {}: Incorrect count of arguments! Expected 2 args",
            FUNC_NAME
        ));
    }

    match (&args[0], &args[1]) {
        (Value::String(target), Value::String(link)) => {
            #[cfg(unix)]
            let result = std::os::unix::fs::symlink(target, link);

            #[cfg(windows)]
            let result = {
                let target_path = PathBuf::from(target);
                if target_path.exists() && target_path.is_dir() {
                    std::os::windows::fs::symlink_dir(target, link)
                } else {
                    std::os::windows::fs::symlink_file(target, link)
                }
            };

            match result {
                Ok(_) => Value::Nil,
                Err(err_text) => Value::Error(error::fmt(
                    LIB_NAME,
                    VALUE_TYPE,
                    FUNC_NAME,
                    "SymlinkError",
                    &err_text.to_string(),
                )),
            }
        }
        (Value::Error(err_text), _) | (_, Value::Error(err_text)) => Value::Error(format!(
            "{}\n{}",
            error::fmt(
                LIB_NAME,
                VALUE_TYPE,
                FUNC_NAME,
                "CatchedError",
                "caught error",
            ),
            err_text
        )),
        _ => Value::Error(error::fmt(
            LIB_NAME,
            VALUE_TYPE,
            FUNC_NAME,
            "TypeError",
            "expected String types for target and link paths",
        )),
    }
}

/// Read the contents of a symlink
pub fn lreadlink(args: Vec<Value>, _scopes: &mut Scopes) -> Value {
    const FUNC_NAME: &str = "readlink";

    if args.len() != 1 {
        return Value::Error(format!(
            "<func> {}: Incorrect count of arguments! Expected 1 arg",
            FUNC_NAME
        ));
    }

    match &args[0] {
        Value::String(path) => {
            let path_buf = PathBuf::from(path);

            if !path_buf.exists() {
                return Value::Error(error::fmt(
                    LIB_NAME,
                    VALUE_TYPE,
                    FUNC_NAME,
                    "PathNotFoundError",
                    &format!("path {} not found!", path),
                ));
            }

            if !path_buf.is_symlink() {
                return Value::Error(error::fmt(
                    LIB_NAME,
                    VALUE_TYPE,
                    FUNC_NAME,
                    "NotASymlinkError",
                    &format!("{} is not a symlink!", path),
                ));
            }

            match fs::read_link(path) {
                Ok(target) => match target.to_str() {
                    Some(target_str) => Value::String(target_str.to_string()),
                    None => Value::Error(error::fmt(
                        LIB_NAME,
                        VALUE_TYPE,
                        FUNC_NAME,
                        "PathConversionError",
                        "Failed to convert path to string",
                    )),
                },
                Err(err_text) => Value::Error(error::fmt(
                    LIB_NAME,
                    VALUE_TYPE,
                    FUNC_NAME,
                    "ReadlinkError",
                    &err_text.to_string(),
                )),
            }
        }
        Value::Error(err_text) => Value::Error(format!(
            "{}\n{}",
            error::fmt(
                LIB_NAME,
                VALUE_TYPE,
                FUNC_NAME,
                "CatchedError",
                "caught error",
            ),
            err_text
        )),
        _ => Value::Error(error::fmt(
            LIB_NAME,
            VALUE_TYPE,
            FUNC_NAME,
            "TypeError",
            "expected String type for path",
        )),
    }
}
