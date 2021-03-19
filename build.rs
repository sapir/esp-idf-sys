use std::{
  env,
  error::Error,
  ffi::OsStr,
  fs::read_to_string,
  io::{BufReader, BufRead, Write},
  os::unix::ffi::OsStrExt,
  path::PathBuf,
  process::{Command, Stdio},
};

use bindgen::EnumVariation;

fn main() -> Result<(), Box<dyn Error>> {
  println!("cargo:rerun-if-changed=src/bindings.h");
  println!("cargo:rerun-if-changed=src/sdkconfig.h");

  let (idf_target, linker) = match env::var("TARGET")?.as_ref() {
    "xtensa-esp32-none-elf" => {
      println!(r#"cargo:rustc-cfg=target_device="esp32""#);
      ("esp32".to_string(), env::var("RUSTC_LINKER").unwrap_or("xtensa-esp32-elf-ld".to_string()))
    },
    "xtensa-esp8266-none-elf" => {
      println!(r#"cargo:rustc-cfg=target_device="esp8266""#);
      ("esp8266".to_string(), env::var("RUSTC_LINKER").unwrap_or("xtensa-lx106-elf-ld".to_string()))
    },
    target => {
      println!("cargo:warning=Generating ESP IDF bindings for target '{}' it not supported. The resulting crate will be empty.", target);
      return Ok(())
    },
  };

  let idf_path = PathBuf::from(env::var("IDF_PATH").expect("IDF_PATH not set"));

  let sysroot = Command::new(linker)
    .arg("--print-sysroot")
    .output()
    .map(|mut output| {
      // Remove newline from end.
      output.stdout.pop();
      PathBuf::from(OsStr::from_bytes(&output.stdout))
        .canonicalize().expect("failed to canonicalize sysroot")
    })
    .expect("failed getting sysroot");

  let component_includes =
    globwalk::GlobWalkerBuilder::from_patterns(
      &idf_path,
      &["components/*/include"],
    )
    .build()?
    .filter_map(Result::ok)
    .map(|d| d.into_path());

  let component_additional_includes = globwalk::GlobWalkerBuilder::from_patterns(
      &idf_path,
      &["components/*/component.mk"],
    )
    .build()?
    .filter_map(Result::ok)
    .flat_map(|makefile| {
      let path = makefile.into_path();
      let component_path = path.parent().unwrap();

      let mut contents = read_to_string(&path).expect("failed reading component.mk").replace("$(info ", "$(warn ");
      // Define these variables since they affect `COMPONENT_ADD_INCLUDEDIRS`.
      contents.insert_str(0, r"
        CONFIG_SYSVIEW_ENABLE :=
        CONFIG_AWS_IOT_SDK :=
        CONFIG_BT_ENABLED :=
        CONFIG_BLUEDROID_ENABLED :=
      ");
      contents.push_str("\n$(info ${COMPONENT_ADD_INCLUDEDIRS})");

      let mut child = Command::new("make")
        .current_dir(&component_path)
        .arg("-f")
        .arg("-")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .env("IDF_TARGET", &idf_target)
        .env("SOC_NAME", &idf_target)
        .env("COMPONENT_PATH", &component_path)
        .spawn()
        .expect("make failed");

      let mut stdin = child.stdin.take().unwrap();
      let stdout = child.stdout.take().unwrap();

      writeln!(stdin, "{}", contents).unwrap();

      BufReader::new(stdout).lines()
        .filter_map(Result::ok)
        .map(|s| s.trim_end().to_string())
        .filter(|s| !s.is_empty())
        .flat_map(|s| {
          let s = s.split(' ');
          let s = s.map(|s| s.to_string());
          s.collect::<Vec<_>>().into_iter()
        })
        .map(move |s| path.parent().unwrap().join(s))
        .filter(|s| s.is_dir())
    });

  let mut includes = component_includes.chain(component_additional_includes)
    .map(|include| format!("-I{}", include.display()))
    .collect::<Vec<_>>();

  includes.sort();
  includes.dedup();

  let bindings = bindgen::Builder::default()
    .use_core()
    .layout_tests(false)
    .ctypes_prefix("libc")
    .default_enum_style(EnumVariation::Rust { non_exhaustive: false } )
    .header("src/bindings.h")
    .clang_arg(format!("--sysroot={}", sysroot.display()))
    .clang_arg(format!("-I{}/include", sysroot.display()))
    .clang_arg("-Isrc")
    .clang_arg("-D__bindgen")
    .clang_args(&["-target", "xtensa"])
    .clang_args(&["-x", "c"])
    .clang_args(includes);

  eprintln!("{:?}", bindings.command_line_flags());

  let out_path = PathBuf::from(env::var("OUT_DIR")?);
  bindings.generate()
    .expect("Failed to generate bindings")
    .write_to_file(out_path.join("bindings.rs"))?;

  Ok(())
}
