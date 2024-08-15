use libsui::Elf;
use libsui::Macho;
use libsui::PortableExecutable;

use libsui::utils;

const HELP: &str = r#"
patchver

USAGE:
    patchver [OPTIONS] <input> <output>

FLAGS:
    -h, --help           Prints help information
    --channel CHANNEL    Update release channel (stable, lts, canary, rc)

ARGS:
    <input>     Input file
    <output>    Output file
"#;

const SECTION: &str = "denover";
const CHANNELS: [&str; 4] = ["stable", "lts", "canary", "rc"];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut pargs = pico_args::Arguments::from_env();

    if pargs.contains(["-h", "--help"]) {
        eprintln!("{}", HELP);
        std::process::exit(1);
    }

    let exe: std::path::PathBuf = pargs.free_from_str().map_err(|_| "Missing input file")?;
    let out: std::path::PathBuf = pargs.free_from_str().map_err(|_| "Missing output file")?;

    let data = pargs.value_from_str::<&str, String>("--channel")?;
    if !CHANNELS.contains(&data.as_str()) {
        eprintln!("Invalid channel. Expected one of: stable, lts, canary, rc");
        std::process::exit(1);
    }

    let exe = std::fs::read(&exe)?;
    let mut out = std::fs::File::create(&out)?;

    let data = data.as_bytes().to_vec();
    if utils::is_pe(&exe) {
        PortableExecutable::from(&exe)?
            .write_resource(SECTION, data)?
            .build(&mut out)?;
    } else if utils::is_macho(&exe) {
        Macho::from(exe)?
            .write_section(SECTION, data)?
            .build_and_sign(&mut out)?;
    } else if utils::is_elf(&exe) {
        Elf::new(&exe).append(SECTION, &data, &mut out)?;
    } else {
        eprintln!("Unsupported file format");
        std::process::exit(1);
    }

    Ok(())
}
