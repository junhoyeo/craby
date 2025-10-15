use crate::utils::{
    collect_packages, get_version_from_commit_message, is_main_ref, run_command,
    validate_package_versions, PackageInfo,
};
use anyhow::Result;

fn publish_napi_package(napi_package: &PackageInfo) -> Result<()> {
    println!("Publishing NAPI package: {}", napi_package.name);

    run_command(
        "yarn",
        &["napi", "prepublish", "-t", "npm", "--no-gh-release"],
        Some(&napi_package.location),
    )?;

    run_command(
        "yarn",
        &["npm", "publish", "--provenance", "--access", "public"],
        Some(&napi_package.location),
    )?;

    Ok(())
}

fn publish_packages(packages: &[PackageInfo]) -> Result<()> {
    for package_info in packages {
        println!("Publishing {}...", package_info.name);

        run_command(
            "yarn",
            &[
                "workspace",
                &package_info.name,
                "npm",
                "publish",
                "--provenance",
                "--access",
                "public",
            ],
            None,
        )?;
    }
    Ok(())
}

pub fn run() -> Result<()> {
    let version = match get_version_from_commit_message()? {
        Some(v) => v,
        None => {
            println!("Not a release, skipping publish");
            return Ok(());
        }
    };

    if !is_main_ref() {
        println!("Not a main branch, skipping publish");
        return Ok(());
    }

    let packages = collect_packages()?;
    validate_package_versions(&packages, &version)?;

    let napi_package = packages
        .iter()
        .find(|p| p.name == "@craby/cli-bindings")
        .ok_or_else(|| anyhow::anyhow!("NAPI package not found, unexpected error"))?;

    let general_packages: Vec<_> = packages
        .iter()
        .filter(|p| p.name != "@craby/cli-bindings")
        .cloned()
        .collect();

    publish_napi_package(napi_package)?;
    publish_packages(&general_packages)?;

    println!("Publish complete");
    Ok(())
}
