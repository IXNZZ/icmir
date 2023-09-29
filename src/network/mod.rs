use bytes::Bytes;
use crossbeam::channel::{Receiver, Sender, unbounded};

pub(crate) fn create_network(addr: &str) -> (Sender<Bytes>, Receiver<Bytes>) {
    let (net_sender, work_receiver):(Sender<Bytes>, Receiver<Bytes>) = unbounded();
    let (work_sender, net_receiver):(Sender<Bytes>, Receiver<Bytes>) = unbounded();

    (net_sender, net_receiver)
}