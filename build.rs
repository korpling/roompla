use actix_web_static_files::NpmBuild;

fn build_npm() -> anyhow::Result<()> {
    NpmBuild::new("./webapp")
        .install()?
        .run("build")?
        .to_resource_dir()
        .build()?;
    Ok(())
}

fn main() {
    build_npm().unwrap();
}
