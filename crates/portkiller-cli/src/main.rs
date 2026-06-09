fn main() {
    let code = portkiller_cli::run_from_args(std::env::args());
    if code != 0 {
        std::process::exit(code);
    }
}
