// This file was generated by gir (https://github.com/gtk-rs/gir @ 32d1716)
// from gir-files (https://github.com/gtk-rs/gir-files @ 62f3bf0)
// DO NOT EDIT

extern crate handy_sys;
extern crate shell_words;
extern crate tempdir;
use std::env;
use std::error::Error;
use std::path::Path;
use std::mem::{align_of, size_of};
use std::process::Command;
use std::str;
use handy_sys::*;

static PACKAGES: &[&str] = &["libhandy-0.0"];

#[derive(Clone, Debug)]
struct Compiler {
    pub args: Vec<String>,
}

impl Compiler {
    pub fn new() -> Result<Compiler, Box<Error>> {
        let mut args = get_var("CC", "cc")?;
        args.push("-Wno-deprecated-declarations".to_owned());
        // For %z support in printf when using MinGW.
        args.push("-D__USE_MINGW_ANSI_STDIO".to_owned());
        args.extend(get_var("CFLAGS", "")?);
        args.extend(get_var("CPPFLAGS", "")?);
        args.extend(pkg_config_cflags(PACKAGES)?);
        Ok(Compiler { args })
    }

    pub fn define<'a, V: Into<Option<&'a str>>>(&mut self, var: &str, val: V) {
        let arg = match val.into() {
            None => format!("-D{}", var),
            Some(val) => format!("-D{}={}", var, val),
        };
        self.args.push(arg);
    }

    pub fn compile(&self, src: &Path, out: &Path) -> Result<(), Box<Error>> {
        let mut cmd = self.to_command();
        cmd.arg(src);
        cmd.arg("-o");
        cmd.arg(out);
        let status = cmd.spawn()?.wait()?;
        if !status.success() {
            return Err(format!("compilation command {:?} failed, {}",
                               &cmd, status).into());
        }
        Ok(())
    }

    fn to_command(&self) -> Command {
        let mut cmd = Command::new(&self.args[0]);
        cmd.args(&self.args[1..]);
        cmd
    }
}

fn get_var(name: &str, default: &str) -> Result<Vec<String>, Box<Error>> {
    match env::var(name) {
        Ok(value) => Ok(shell_words::split(&value)?),
        Err(env::VarError::NotPresent) => Ok(shell_words::split(default)?),
        Err(err) => Err(format!("{} {}", name, err).into()),
    }
}

fn pkg_config_cflags(packages: &[&str]) -> Result<Vec<String>, Box<Error>> {
    if packages.is_empty() {
        return Ok(Vec::new());
    }
    let mut cmd = Command::new("pkg-config");
    cmd.arg("--cflags");
    cmd.args(packages);
    let out = cmd.output()?;
    if !out.status.success() {
        return Err(format!("command {:?} returned {}",
                           &cmd, out.status).into());
    }
    let stdout = str::from_utf8(&out.stdout)?;
    Ok(shell_words::split(stdout.trim())?)
}


#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Layout {
    size: usize,
    alignment: usize,
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
struct Results {
    /// Number of successfully completed tests.
    passed: usize,
    /// Total number of failed tests (including those that failed to compile).
    failed: usize,
    /// Number of tests that failed to compile.
    failed_to_compile: usize,
}

impl Results {
    fn record_passed(&mut self) {
        self.passed += 1;
    }
    fn record_failed(&mut self) {
        self.failed += 1;
    }
    fn record_failed_to_compile(&mut self) {
        self.failed += 1;
        self.failed_to_compile += 1;
    }
    fn summary(&self) -> String {
        format!(
            "{} passed; {} failed (compilation errors: {})",
            self.passed,
            self.failed,
            self.failed_to_compile)
    }
    fn expect_total_success(&self) {
        if self.failed == 0 {
            println!("OK: {}", self.summary());
        } else {
            panic!("FAILED: {}", self.summary());
        };
    }
}

#[test]
fn cross_validate_constants_with_c() {
    let tmpdir = tempdir::TempDir::new("abi").expect("temporary directory");
    let cc = Compiler::new().expect("configured compiler");

    assert_eq!("1",
               get_c_value(tmpdir.path(), &cc, "1").expect("C constant"),
               "failed to obtain correct constant value for 1");

    let mut results : Results = Default::default();
    for (i, &(name, rust_value)) in RUST_CONSTANTS.iter().enumerate() {
        match get_c_value(tmpdir.path(), &cc, name) {
            Err(e) => {
                results.record_failed_to_compile();
                eprintln!("{}", e);
            },
            Ok(ref c_value) => {
                if rust_value == c_value {
                    results.record_passed();
                } else {
                    results.record_failed();
                    eprintln!("Constant value mismatch for {}\nRust: {:?}\nC:    {:?}",
                              name, rust_value, c_value);
                }
            }
        };
        if (i + 1) % 25 == 0 {
            println!("constants ... {}", results.summary());
        }
    }
    results.expect_total_success();
}

#[test]
fn cross_validate_layout_with_c() {
    let tmpdir = tempdir::TempDir::new("abi").expect("temporary directory");
    let cc = Compiler::new().expect("configured compiler");

    assert_eq!(Layout {size: 1, alignment: 1},
               get_c_layout(tmpdir.path(), &cc, "char").expect("C layout"),
               "failed to obtain correct layout for char type");

    let mut results : Results = Default::default();
    for (i, &(name, rust_layout)) in RUST_LAYOUTS.iter().enumerate() {
        match get_c_layout(tmpdir.path(), &cc, name) {
            Err(e) => {
                results.record_failed_to_compile();
                eprintln!("{}", e);
            },
            Ok(c_layout) => {
                if rust_layout == c_layout {
                    results.record_passed();
                } else {
                    results.record_failed();
                    eprintln!("Layout mismatch for {}\nRust: {:?}\nC:    {:?}",
                              name, rust_layout, &c_layout);
                }
            }
        };
        if (i + 1) % 25 == 0 {
            println!("layout    ... {}", results.summary());
        }
    }
    results.expect_total_success();
}

fn get_c_layout(dir: &Path, cc: &Compiler, name: &str) -> Result<Layout, Box<Error>> {
    let exe = dir.join("layout");
    let mut cc = cc.clone();
    cc.define("ABI_TYPE_NAME", name);
    cc.compile(Path::new("tests/layout.c"), &exe)?;

    let mut abi_cmd = Command::new(exe);
    let output = abi_cmd.output()?;
    if !output.status.success() {
        return Err(format!("command {:?} failed, {:?}",
                           &abi_cmd, &output).into());
    }

    let stdout = str::from_utf8(&output.stdout)?;
    let mut words = stdout.trim().split_whitespace();
    let size = words.next().unwrap().parse().unwrap();
    let alignment = words.next().unwrap().parse().unwrap();
    Ok(Layout {size, alignment})
}

fn get_c_value(dir: &Path, cc: &Compiler, name: &str) -> Result<String, Box<Error>> {
    let exe = dir.join("constant");
    let mut cc = cc.clone();
    cc.define("ABI_CONSTANT_NAME", name);
    cc.compile(Path::new("tests/constant.c"), &exe)?;

    let mut abi_cmd = Command::new(exe);
    let output = abi_cmd.output()?;
    if !output.status.success() {
        return Err(format!("command {:?} failed, {:?}",
                           &abi_cmd, &output).into());
    }

    let output = str::from_utf8(&output.stdout)?.trim();
    if !output.starts_with("###gir test###") ||
       !output.ends_with("###gir test###") {
        return Err(format!("command {:?} return invalid output, {:?}",
                           &abi_cmd, &output).into());
    }

    Ok(String::from(&output[14..(output.len() - 14)]))
}

const RUST_LAYOUTS: &[(&str, Layout)] = &[
    ("HdyActionRow", Layout {size: size_of::<HdyActionRow>(), alignment: align_of::<HdyActionRow>()}),
    ("HdyActionRowClass", Layout {size: size_of::<HdyActionRowClass>(), alignment: align_of::<HdyActionRowClass>()}),
    ("HdyArrows", Layout {size: size_of::<HdyArrows>(), alignment: align_of::<HdyArrows>()}),
    ("HdyArrowsClass", Layout {size: size_of::<HdyArrowsClass>(), alignment: align_of::<HdyArrowsClass>()}),
    ("HdyArrowsDirection", Layout {size: size_of::<HdyArrowsDirection>(), alignment: align_of::<HdyArrowsDirection>()}),
    ("HdyColumnClass", Layout {size: size_of::<HdyColumnClass>(), alignment: align_of::<HdyColumnClass>()}),
    ("HdyComboRow", Layout {size: size_of::<HdyComboRow>(), alignment: align_of::<HdyComboRow>()}),
    ("HdyComboRowClass", Layout {size: size_of::<HdyComboRowClass>(), alignment: align_of::<HdyComboRowClass>()}),
    ("HdyDialer", Layout {size: size_of::<HdyDialer>(), alignment: align_of::<HdyDialer>()}),
    ("HdyDialerButton", Layout {size: size_of::<HdyDialerButton>(), alignment: align_of::<HdyDialerButton>()}),
    ("HdyDialerButtonClass", Layout {size: size_of::<HdyDialerButtonClass>(), alignment: align_of::<HdyDialerButtonClass>()}),
    ("HdyDialerClass", Layout {size: size_of::<HdyDialerClass>(), alignment: align_of::<HdyDialerClass>()}),
    ("HdyDialerCycleButton", Layout {size: size_of::<HdyDialerCycleButton>(), alignment: align_of::<HdyDialerCycleButton>()}),
    ("HdyDialerCycleButtonClass", Layout {size: size_of::<HdyDialerCycleButtonClass>(), alignment: align_of::<HdyDialerCycleButtonClass>()}),
    ("HdyEnumValueObjectClass", Layout {size: size_of::<HdyEnumValueObjectClass>(), alignment: align_of::<HdyEnumValueObjectClass>()}),
    ("HdyExpanderRow", Layout {size: size_of::<HdyExpanderRow>(), alignment: align_of::<HdyExpanderRow>()}),
    ("HdyExpanderRowClass", Layout {size: size_of::<HdyExpanderRowClass>(), alignment: align_of::<HdyExpanderRowClass>()}),
    ("HdyFold", Layout {size: size_of::<HdyFold>(), alignment: align_of::<HdyFold>()}),
    ("HdyHeaderGroup", Layout {size: size_of::<HdyHeaderGroup>(), alignment: align_of::<HdyHeaderGroup>()}),
    ("HdyHeaderGroupClass", Layout {size: size_of::<HdyHeaderGroupClass>(), alignment: align_of::<HdyHeaderGroupClass>()}),
    ("HdyLeaflet", Layout {size: size_of::<HdyLeaflet>(), alignment: align_of::<HdyLeaflet>()}),
    ("HdyLeafletChildTransitionType", Layout {size: size_of::<HdyLeafletChildTransitionType>(), alignment: align_of::<HdyLeafletChildTransitionType>()}),
    ("HdyLeafletClass", Layout {size: size_of::<HdyLeafletClass>(), alignment: align_of::<HdyLeafletClass>()}),
    ("HdyLeafletModeTransitionType", Layout {size: size_of::<HdyLeafletModeTransitionType>(), alignment: align_of::<HdyLeafletModeTransitionType>()}),
    ("HdySearchBar", Layout {size: size_of::<HdySearchBar>(), alignment: align_of::<HdySearchBar>()}),
    ("HdySearchBarClass", Layout {size: size_of::<HdySearchBarClass>(), alignment: align_of::<HdySearchBarClass>()}),
    ("HdyTitleBarClass", Layout {size: size_of::<HdyTitleBarClass>(), alignment: align_of::<HdyTitleBarClass>()}),
];

const RUST_CONSTANTS: &[(&str, &str)] = &[
    ("(gint) HDY_ARROWS_DIRECTION_DOWN", "1"),
    ("(gint) HDY_ARROWS_DIRECTION_LEFT", "2"),
    ("(gint) HDY_ARROWS_DIRECTION_RIGHT", "3"),
    ("(gint) HDY_ARROWS_DIRECTION_UP", "0"),
    ("(gint) HDY_FOLD_FOLDED", "1"),
    ("(gint) HDY_FOLD_UNFOLDED", "0"),
    ("(gint) HDY_LEAFLET_CHILD_TRANSITION_TYPE_CROSSFADE", "1"),
    ("(gint) HDY_LEAFLET_CHILD_TRANSITION_TYPE_NONE", "0"),
    ("(gint) HDY_LEAFLET_CHILD_TRANSITION_TYPE_OVER", "3"),
    ("(gint) HDY_LEAFLET_CHILD_TRANSITION_TYPE_SLIDE", "2"),
    ("(gint) HDY_LEAFLET_MODE_TRANSITION_TYPE_NONE", "0"),
    ("(gint) HDY_LEAFLET_MODE_TRANSITION_TYPE_SLIDE", "1"),
];


