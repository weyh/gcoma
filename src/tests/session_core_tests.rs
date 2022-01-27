use crate::session_core::connection_type::ConnectionType;
use crate::session_core::session::Session;

#[test]
fn ip_test1() {
    let session = Session::new(
        "session".to_string(),
        "192.168.0.1:23".to_string(),
        ConnectionType::SSH,
    );

    assert_eq!(session.get_ip(), "192.168.0.1");
}

#[test]
fn ip_test2() {
    let session = Session::new(
        "session".to_string(),
        "user@192.168.0.1:23".to_string(),
        ConnectionType::SSH,
    );

    assert_eq!(session.get_ip(), "192.168.0.1");
}

#[test]
fn ip_test3() {
    let session = Session::new(
        "session".to_string(),
        "user@192.168.0.1".to_string(),
        ConnectionType::SSH,
    );

    assert_eq!(session.get_ip(), "192.168.0.1");
}

#[test]
fn ip_test4() {
    let session = Session::new(
        "session".to_string(),
        "192.168.0.1".to_string(),
        ConnectionType::SSH,
    );

    assert_eq!(session.get_ip(), "192.168.0.1");
}

#[test]
fn port_test1() {
    let session = Session::new(
        "session".to_string(),
        "192.168.0.1:23".to_string(),
        ConnectionType::SSH,
    );

    assert_eq!(session.get_port(), "23");
}

#[test]
fn port_test2() {
    let session = Session::new(
        "session".to_string(),
        "user@192.168.0.1:2222".to_string(),
        ConnectionType::SSH,
    );
    assert_eq!(session.get_port(), "2222");
}

#[test]
fn port_test3() {
    let session = Session::new(
        "session".to_string(),
        "user@192.168.0.1".to_string(),
        ConnectionType::SSH,
    );

    assert_eq!(session.get_port(), "22");
}

#[test]
fn port_test4() {
    let session = Session::new(
        "session".to_string(),
        "192.168.0.1".to_string(),
        ConnectionType::SSH,
    );

    assert_eq!(session.get_port(), "22");
}
