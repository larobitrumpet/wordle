use std::env;

fn main() {
    let mut args = env::args();
    args.next();
    if let Some(arg) = args.next() {
        match &arg[..] {
            "run" => wordle::run(),
            "test" => wordle::run_tests(),
            "play" => wordle::play(),
            _ => println!("Unknow argument: {}", arg),
        }
    } else {
        wordle::run();
    }
}
