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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut pargs = pico_args::Arguments::from_env();

    if pargs.contains(["-h", "--help"]) {
        eprintln!("{}", HELP);
        std::process::exit(1);
    }

    let exe: std::path::PathBuf = pargs.free_from_str().map_err(|_| "Missing input file")?;
    let out: std::path::PathBuf = pargs.free_from_str().map_err(|_| "Missing output file")?;

    let data = pargs.value_from_str::<&str, String>("--channel")?;

    let exe = std::fs::read(&exe)?;
    let mut out = std::fs::File::create(&out)?;
    patchver::patchver(exe, data, &mut out)?;

    Ok(())
}
