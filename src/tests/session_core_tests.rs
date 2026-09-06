use crate::session_core::connection_type::ConnectionType;
use crate::session_core::session::Session;
use std::ffi::OsStr;

macro_rules! SESSION_NAME {
    () => {
        "TEST_SESSION".to_string()
    };
}

#[test]
fn ip_test1() {
    let session = Session::new(
        SESSION_NAME!(),
        "192.168.0.1:23".to_string(),
        ConnectionType::SSH,
    );

    assert_eq!(session.get_ip(), "192.168.0.1");
}

#[test]
fn ip_test2() {
    let session = Session::new(
        SESSION_NAME!(),
        "user@192.168.0.1:23".to_string(),
        ConnectionType::SSH,
    );

    assert_eq!(session.get_ip(), "192.168.0.1");
}

#[test]
fn ip_test3() {
    let session = Session::new(
        SESSION_NAME!(),
        "user@192.168.0.1".to_string(),
        ConnectionType::SSH,
    );

    assert_eq!(session.get_ip(), "192.168.0.1");
}

#[test]
fn ip_test4() {
    let session = Session::new(
        SESSION_NAME!(),
        "192.168.0.1".to_string(),
        ConnectionType::SSH,
    );

    assert_eq!(session.get_ip(), "192.168.0.1");
}

#[test]
fn port_test1() {
    let session = Session::new(
        SESSION_NAME!(),
        "192.168.0.1:23".to_string(),
        ConnectionType::SSH,
    );

    assert_eq!(session.get_port(), "23");
}

#[test]
fn port_test2() {
    let session = Session::new(
        SESSION_NAME!(),
        "user@192.168.0.1:2222".to_string(),
        ConnectionType::SSH,
    );
    assert_eq!(session.get_port(), "2222");
}

#[test]
fn port_test3() {
    let session = Session::new(
        SESSION_NAME!(),
        "user@192.168.0.1".to_string(),
        ConnectionType::SSH,
    );

    assert_eq!(session.get_port(), "22");
}

#[test]
fn port_test4() {
    let session = Session::new(
        SESSION_NAME!(),
        "192.168.0.1".to_string(),
        ConnectionType::SSH,
    );

    assert_eq!(session.get_port(), "22");
}

#[test]
fn user_name_test1() {
    let session = Session::new(
        SESSION_NAME!(),
        "user_name@192.168.0.1:2222".to_string(),
        ConnectionType::SSH,
    );

    assert_eq!(session.get_user_name(), "user_name");
}

#[test]
fn user_name_test2() {
    let session = Session::new(
        SESSION_NAME!(),
        "192.168.0.1:2222".to_string(),
        ConnectionType::SSH,
    );

    assert_eq!(session.get_user_name(), "");
}

#[test]
fn command_session_launches_the_executable_with_its_arguments() {
    let session = Session::new(
        SESSION_NAME!(),
        "scp -P 2222 'source file' user@example.com:/tmp".to_string(),
        ConnectionType::Command,
    );
    let command = session.build_command().unwrap();

    assert_eq!(command.get_program(), OsStr::new("scp"));
    assert_eq!(
        command.get_args().collect::<Vec<_>>(),
        vec![
            OsStr::new("-P"),
            OsStr::new("2222"),
            OsStr::new("source file"),
            OsStr::new("user@example.com:/tmp"),
        ]
    );
    assert_eq!(
        session.command_display().unwrap(),
        "scp -P 2222 'source file' user@example.com:/tmp"
    );
}

#[test]
fn command_session_requires_an_executable() {
    let session = Session::new(SESSION_NAME!(), "  ".to_string(), ConnectionType::Command);
    let error = session.build_command().unwrap_err();
    assert_eq!(error.kind(), std::io::ErrorKind::InvalidInput);
}

#[test]
fn ssh_session_display_matches_the_command_that_will_run() {
    let session = Session::new(
        SESSION_NAME!(),
        "user@192.168.0.1:2222".to_string(),
        ConnectionType::SSH,
    );

    assert_eq!(
        session.command_display().unwrap(),
        "ssh 192.168.0.1 -p 2222 -l user"
    );
}
