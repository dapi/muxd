use muxd::error::MuxdError;

#[test]
fn error_variants_map_to_stable_exit_codes() {
    assert_eq!(MuxdError::InvalidInput("bad input").exit_code(), 1);
    assert_eq!(
        MuxdError::BackendUnavailable("missing backend".to_string()).exit_code(),
        2
    );
    assert_eq!(
        MuxdError::ResourceUnavailable("missing session".to_string()).exit_code(),
        3
    );
    assert_eq!(MuxdError::LaunchFailed("boom".to_string()).exit_code(), 4);
}
