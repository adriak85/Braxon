use std::io;

fn main() -> io::Result<()> {
    eprintln!(
        "nsq-compile disabled: legacy compile path still lowers canonical NSQ \
into derived machine forms. Lock nsq-core + parser/build/eval to native `nu` \
substrate before re-enabling compile."
    );
    std::process::exit(2);
}
