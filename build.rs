use std::io::Result;
fn main() -> Result<()> {
    // https://www.ubitools.com/en/prost-build/
    let mut config = prost_build::Config::new();

    // config.out_dir("./src/");
    config.default_package_filename("findify");
    config.type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]");

    config.compile_protos(&[
        // Via git submodule
        "src/active-sync/activesync.proto",
        "src/active-sync/config.proto",
        "src/active-sync/content.proto",
        "src/active-sync/field.proto",
        "src/active-sync/itemgroup.proto",

        // Copied from Lucy
        "src/protobuf/indexSync.proto"
    ], &["src/"])?;

    Ok(())
}
