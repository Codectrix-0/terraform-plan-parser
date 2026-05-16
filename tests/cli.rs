use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

fn temp_dir(name: &str) -> PathBuf {
    let mut dir = env::temp_dir();

    // On Windows, the `\\?\` prefix breaks PATH lookups for child processes.
    // Stripping it allows the mock `terraform.bat` to be found in the PATH.
    #[cfg(windows)]
    {
        let dir_str = dir.to_string_lossy();
        if dir_str.starts_with(r"\\?\") {
            dir = PathBuf::from(&dir_str[4..]);
        }
    }

    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time is after unix epoch")
        .as_nanos();
    dir.push(format!(
        "terraform_plan_parser_{name}_{}_{}",
        std::process::id(),
        nanos
    ));
    fs::create_dir_all(&dir).expect("create temp dir");
    dir
}
