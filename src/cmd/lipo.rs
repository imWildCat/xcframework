use crate::conf::Configuration;
use anyhow::Result;
use rustup_target::Triple;

pub fn assemble_libs(conf: &Configuration) -> Result<Vec<String>> {
    fs_err::create_dir_all(&conf.build_dir.join("libs"))?;

    let mut libs = vec![];
    libs.push(join_or_copy(conf, &conf.cargo_section.iOS_targets, "ios")?);
    libs.push(join_or_copy(
        conf,
        &conf.cargo_section.iOS_simulator_targets,
        "ios_sim",
    )?);
    libs.push(join_or_copy(
        conf,
        &conf.cargo_section.macOS_targets,
        "macos",
    )?);

    Ok(libs)
}

fn join_or_copy(conf: &Configuration, targets: &[Triple], name: &str) -> Result<String> {
    if targets.len() == 1 {
        single_copy(conf, &targets[0], name)
    } else {
        lipo_join(conf, targets, name)
    }
}

fn lipo_join(conf: &Configuration, targets: &[Triple], name_ext: &str) -> Result<String> {
    let profile = conf.cli.profile.as_deref().unwrap_or("debug");
    let dir = &conf.build_dir;
    let name = &conf.lib_name.replace('-', "_");
    let ending = conf.lib_type.file_ending();

    let mut args = vec!["-create".to_string()];
    for target in targets {
        args.push(format!("target/{target}/{profile}/lib{name}.{ending}"));
    }

    args.push("-output".into());
    let out = format!("{dir}/libs/lib{name}_{name_ext}.{ending}");
    args.push(out.clone());

    super::run("lipo", &args)?;
    Ok(out)
}

fn single_copy(conf: &Configuration, target: &Triple, name_ext: &str) -> Result<String> {
    let profile = conf.profile();
    let ending = conf.lib_type.file_ending();
    let dir = &conf.build_dir;
    let name = &conf.lib_name.replace('-', "_");

    let src = format!("target/{target}/{profile}/lib{name}.{ending}",);
    let dest = format!("{dir}/libs/lib{name}_{name_ext}.{ending}");
    fs_err::copy(src, &dest)?;
    Ok(dest)
}