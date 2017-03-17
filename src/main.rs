extern crate indigo;
extern crate getopts;

use getopts::Options;
use indigo::events::VectorStream;
use indigo::events::name_hint::NameHint;
use indigo::events::groups::GroupPipeline;

use std::env;

struct CommandParameters {
    group_path: String,
    names_path: String
}

fn get_args(args: Vec<String>) -> Result<CommandParameters, i32> {
    let mut opts = Options::new();

    opts.optflagopt("g", "group", "path to group file", "PATH_TO_GROUP_FILE");
    opts.optflagopt("n", "name", "path to names file", "PATH_TO_NAMES_FILE");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(failure) => panic!(failure.to_string())
    };

    let names_path = try!(matches.opt_str("n").ok_or(1));
    let group_path = try!(matches.opt_str("g").ok_or(2));

    Ok(CommandParameters {
        group_path: group_path,
        names_path: names_path
    })
}

fn print_usage() {
    println!("usage: indigo -n /path/to/names -g /path/to/groups");
}

fn main() {
    let mut opts = Options::new();

    opts.optflagopt("g", "group", "path to group file", "PATH_TO_GROUP_FILE");
    opts.optflagopt("n", "name", "path to names file", "PATH_TO_NAMES_FImmLE");

    match get_args(env::args().collect()) {
        Ok(parameters) => {
            let hint: String = NameHint::ld_string(&parameters.names_path).unwrap();
            let stream = VectorStream::from(
                &hint,
                &parameters.group_path);

            let hist = GroupPipeline::new(stream, 10, 5);
            hist.mine();
        }
        Err(_) => print_usage()
    }
}
