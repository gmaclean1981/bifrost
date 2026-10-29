//! Exercise the real query handler and inspect actual UDP replies.
//! Run only inside the isolated network namespace created by test-isolated.sh.
use super::*;
use std::time::{Duration, Instant};

fn exchange(names: &[&str], known: bool) -> Option<DnsIncoming> {
    assert_eq!(std::env::var("BIFROST_ISOLATED_TEST").as_deref(), Ok("1"));
    let signal = MioUdpSocket::bind("127.0.0.1:0".parse().unwrap()).unwrap();
    let mut daemon = Zeroconf::new(signal, Poll::new().unwrap());
    let intf = daemon.intf_socks.keys().find(|i| i.ip().is_ipv4()).unwrap().clone();
    let mut svc = ServiceInfo::new("_hue._tcp.local.", "bifrost-test", "bifrost-test.local.",
        intf.ip(), 443, &[("bridgeid", "001122FFFE334455"), ("modelid", "BSB002")][..]).unwrap();
    svc.set_status(&intf, ServiceStatus::Announced);
    let fullname = svc.get_fullname().to_owned();
    daemon.my_services.insert(fullname.clone(), svc);
    let mut query = DnsOutgoing::new(0);
    for name in names { query.add_question(name, RRType::PTR); }
    if known {
        query.add_answer_box(Box::new(DnsPointer::new("_hue._tcp.local.", RRType::PTR,
            CLASS_IN, 4500, fullname)));
    }
    let incoming = DnsIncoming::new(query.to_data_on_wire().remove(0)).unwrap();
    daemon.handle_query(incoming, &intf);
    let socket = daemon.intf_socks.get(&intf).unwrap();
    let deadline = Instant::now() + Duration::from_millis(250);
    let mut buf = [0u8; 9000];
    while Instant::now() < deadline {
        match socket.recv_from(&mut buf) {
            Ok((n, _)) => {
                let packet = DnsIncoming::new(buf[..n].to_vec()).unwrap();
                // Socket initialization emits a looped-back query to check multicast support.
                if packet.is_response() { return Some(packet); }
            }
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => std::thread::sleep(Duration::from_millis(5)),
            Err(e) => panic!("receive failed: {}", e),
        }
    }
    None
}

#[test]
fn unrelated_query_is_silent() {
    assert!(exchange(&["_airplay._tcp.local."], false).is_none());
}

#[test]
fn hue_query_retains_ptr_srv_txt_and_address() {
    let reply = exchange(&["_hue._tcp.local."], false).expect("Hue discovery response");
    assert_eq!(reply.answers().len(), 1);
    assert_eq!(reply.answers()[0].get_type(), RRType::PTR);
    for ty in [RRType::SRV, RRType::TXT, RRType::A] {
        assert!(reply.additionals().iter().any(|r| r.get_type() == ty));
    }
}

#[test]
fn service_enumeration_still_works() {
    let reply = exchange(&["_services._dns-sd._udp.local."], false).expect("service enumeration response");
    assert_eq!(reply.answers().len(), 1);
    assert_eq!(reply.answers()[0].get_type(), RRType::PTR);
}

#[test]
fn mixed_query_keeps_the_matching_answer() {
    let reply = exchange(&["_airplay._tcp.local.", "_hue._tcp.local."], false).unwrap();
    assert_eq!(reply.answers().len(), 1);
}

#[test]
fn suppressed_known_answer_is_silent() {
    assert!(exchange(&["_hue._tcp.local."], true).is_none());
}
