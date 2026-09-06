use crate::session_core::connection_type::ConnectionType;
use crate::session_core::session::Session;
use crate::session_core::session_group::SessionGroup;
use crate::ui::config::Config;

#[test]
fn session_config_round_trip() {
    // Create a sample config with session groups and sessions
    let session1 = Session::new(
        "Test Session 1".to_string(),
        "user@192.168.1.1:2222".to_string(),
        ConnectionType::SSH,
    );

    let session2 = Session::new(
        "Test Session 2".to_string(),
        "192.168.1.2:23".to_string(),
        ConnectionType::Telnet,
    );

    let session_group = SessionGroup::new("Test Group".to_string(), vec![session1, session2]);

    let config = Config {
        version: "2.2.1".to_string(),
        session_groups: vec![session_group],
    };

    // Serialize to JSON
    let json = serde_json::to_string(&config).unwrap();

    // Deserialize back
    let config2: Config = serde_json::from_str(&json).unwrap();

    // Verify round-trip
    assert_eq!(config.version, config2.version);
    assert_eq!(config.session_groups.len(), config2.session_groups.len());
    assert_eq!(
        config.session_groups[0].name,
        config2.session_groups[0].name
    );
    assert_eq!(
        config.session_groups[0].sessions.len(),
        config2.session_groups[0].sessions.len()
    );
    assert_eq!(
        config.session_groups[0].sessions[0].name,
        config2.session_groups[0].sessions[0].name
    );
    assert_eq!(
        config.session_groups[0].sessions[0].data,
        config2.session_groups[0].sessions[0].data
    );
    assert_eq!(
        config.session_groups[0].sessions[0].connection_type,
        config2.session_groups[0].sessions[0].connection_type
    );

    // Verify the serialized JSON can be re-serialized to the same value
    let json2 = serde_json::to_string(&config2).unwrap();
    let config3: Config = serde_json::from_str(&json2).unwrap();
    assert_eq!(config2.version, config3.version);
}

#[test]
fn command_config_round_trip_and_legacy_types_deserialize() {
    let json = r#"
    {
      "version": "2.2.1",
      "session_groups": [{
        "name": "Test Group",
        "sessions": [
          {"name": "Legacy", "data": "host", "connection_type": "SSH"},
          {"name": "Command", "data": "echo hello", "connection_type": "command"}
        ]
      }]
    }"#;

    let config: Config = serde_json::from_str(json).unwrap();
    let sessions = &config.session_groups[0].sessions;
    assert_eq!(sessions[0].connection_type, ConnectionType::SSH);
    assert_eq!(sessions[1].connection_type, ConnectionType::Command);

    let serialized = serde_json::to_string(&config).unwrap();
    assert!(serialized.contains("\"connection_type\":\"SSH\""));
    assert!(serialized.contains("\"connection_type\":\"Command\""));
}
