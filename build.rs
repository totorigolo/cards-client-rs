fn export_env_variable(key: impl AsRef<str>, value: impl AsRef<str>) {
    println!("cargo:rustc-env={}={}", key.as_ref(), value.as_ref());
}

fn main() {
    // Make the Git commit hash accessible during compilation.
    let git_commit_hash = git2::Repository::open(".")
        .and_then(|repository| {
            repository
                .revparse_single("HEAD")
                .and_then(|obj| obj.short_id().map(|buf| buf.as_str().unwrap().to_string()))
        })
        .unwrap_or("release".to_string());
    export_env_variable("GIT_COMMIT_HASH", git_commit_hash);
}
