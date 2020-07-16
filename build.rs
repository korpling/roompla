#[cfg(npm)]
fn build_npm() -> anyhow::Result<()> {
    use actix_web_static_files::NpmBuild;
    NpmBuild::new("./webapp")
        .install()?
        .run("build")?
        .to_resource_dir()
        .build()?;
    Ok(())
}

#[cfg(not(npm))]
fn build_npm() -> anyhow::Result<()> {
    use actix_web_static_files::resource_dir;
    resource_dir("./webapp/dist/").build()?;
    Ok(())
}

fn main() {
    build_npm().unwrap();
}
