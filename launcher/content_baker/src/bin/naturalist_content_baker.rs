use std::{
    env, fs, io,
    path::{Path, PathBuf},
};

fn launcher_path(key: &str) -> io::Result<PathBuf> {
    env::var_os(key)
        .map(PathBuf::from)
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, key))
}

// The launcher can clone the previously selected version's resources on
// upgrade. Replace our own alias in the new version without writing through a
// hard link or symlink to the older build.
fn install_recipe_alias(source: &Path, alias: &Path) -> io::Result<()> {
    let mut input = fs::File::open(source)?;
    let staged = alias.with_extension(format!("tmp.{}", std::process::id()));
    let mut output = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&staged)?;
    let result = io::copy(&mut input, &mut output).and_then(|_| output.sync_all());
    drop(output);
    let result = result.and_then(|()| fs::rename(&staged, alias));
    if result.is_err() {
        let _ = fs::remove_file(&staged);
    }
    result
}

fn main() -> io::Result<()> {
    let version = launcher_path("HORIZON_NATIVE_VERSION_DIR")?;
    let content = version.join("native-content").join("naturalist");
    let bundle = launcher_path("HORIZON_NATIVE_BUNDLE_DATA_DIR")?;
    let file = fs::File::open(bundle.join("content.zip"))?;
    let mut assets = zip::ZipArchive::new(file)?;
    assets.extract(&content)?;
    if !content.join("horizon-mod.toml").is_file() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "missing Naturalist content manifest",
        ));
    }
    let output = version.join(".local/horizon-client/26.3/data/minecraft/recipe/brewing");
    if !output.is_dir() || !output.canonicalize()?.starts_with(version.canonicalize()?) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "the built version has no private brewing folder",
        ));
    }
    for recipe in fs::read_dir(content.join("data/naturalist/recipe/brewing"))? {
        let recipe = recipe?;
        if !recipe.file_type()?.is_file()
            || recipe.path().extension().and_then(|ext| ext.to_str()) != Some("json")
        {
            continue;
        }
        let alias = output.join(format!(
            "naturalist_{}",
            recipe.file_name().to_string_lossy()
        ));
        install_recipe_alias(&recipe.path(), &alias)?;
    }

    Ok(())
}
