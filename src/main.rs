use crate::core::api::{pbrt_cleanup, pbrt_init, APIState};

mod core;

struct Options {
    ncores: u32,
    nthreads: u32,
    outfile: String,
    quick: bool,
    verbose: bool,
    help: bool,
}

fn main() {
    // CLI options
    // --ncores
    // --nthreads
    // --outfile
    // --quick
    // --verbose
    // --help
    // rest of input to filenames

    let filenames: Vec<String> = vec![];

    let mut current_state = APIState::Uninitialized;

    pbrt_init(&mut current_state).unwrap();

    // process scene
    if filenames.len() == 0 {
        // ParseFile("-")
    } else {
        for filename in filenames {
            // ParseFile(filename)
        }
    }

    // parse pbrt file
    // cleanup
    pbrt_cleanup(&mut current_state).unwrap();
}
