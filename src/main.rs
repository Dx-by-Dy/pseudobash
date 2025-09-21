use pseudobash::{global_struct::GS, listener::Listener};

fn main() {
    let mut gs = GS::default();
    let mut listener = Listener::default();
    listener.start(&mut gs);
}
