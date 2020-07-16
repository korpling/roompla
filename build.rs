use actix_web_static_files::resource_dir;
use actix_web_static_files::NpmBuild;

fn build_npm() -> anyhow::Result<()> {
    if std::env::var("CARGO_FEATURE_NPM").is_ok() {
        println!("Compiling web application to ./webapp/dist/");
        NpmBuild::new("./webapp")
            .install()?
            .run("build")?
            .target("./webapp/dist")
            .to_resource_dir()
            .build()?;
    } else {
        println!("Using pre-compiled web application from ./webapp/dist/");
        resource_dir("./webapp/dist/").build()?;
    }
    Ok(())
}

fn main() {
    build_npm().unwrap();
}
