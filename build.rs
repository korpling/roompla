use actix_web_static_files::resource_dir;
use actix_web_static_files::NpmBuild;

fn build_npm() -> anyhow::Result<()> {
    if cfg!(npm) {
        resource_dir("./webapp/dist/").build()?;
    } else {
        NpmBuild::new("./webapp")
            .install()?
            .run("build")?
            .target("./webapp/dist")
            .to_resource_dir()
            .build()?;
    }
    Ok(())
}

fn main() {
    build_npm().unwrap();
}
