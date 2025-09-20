use pseudobash::{global_struct::GS, listener::Listener};

fn main() {
    let mut gs = GS::default();
    Listener::start(&mut gs);
}
