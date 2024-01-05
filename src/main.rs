use std::env;
use std::process::Command;

fn main() {
    let config = parse_config();
    let app = config.app;
    let verb = config.verb;
    let env = config.env;
    let op_uri = format!("op://credentials/{app}/{env}/RAILS_MASTER_KEY");
    Command::new("op")
        .env("RAILS_MASTER_KEY", op_uri)
        .arg("run")
        .arg("--")
        .arg("bin/rails")
        .arg(format!("credentials:{verb}"))
        .arg("--environment")
        .arg(env)
        .spawn()
        .expect("Failed to run command");
}

struct Config {
    app: String,
    verb: String,
    env: String,
}

fn parse_config() -> Config {
    let args: Vec<String> = env::args().collect();
    let app = app();
    let verb = args.get(1).expect("Missing verb").to_string();
    let env = args.get(2).expect("Missing env name").to_string();

    Config {
        app,
        verb,
        env,
    }
}

fn app() -> String {
    let current_dir = env::current_dir().expect("Failed to get current dir");
    let dir_name = current_dir.file_name()
        .expect("Failed to get directory name")
        .to_str()
        .expect("Failed to convert to str");
    dir_name.to_string()
}
