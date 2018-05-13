use super::*;

fn assert_send<T: Send>() {}
fn assert_sync<T: Sync>() {}

#[test]
fn options_is_send() {
    assert_send::<Options>();
}

#[test]
fn options_is_sync() {
    assert_sync::<Options>();
}

#[test]
fn resolver_is_send() {
    assert_send::<Resolver>();
}

#[test]
fn resolver_is_sync() {
    assert_sync::<Resolver>();
}

#[test]
fn blocking_resolver_is_send() {
    assert_send::<BlockingResolver>();
}

#[test]
fn blocking_resolver_is_sync() {
    assert_sync::<BlockingResolver>();
}

#[test]
fn future_resolver_is_send() {
    assert_send::<FutureResolver>();
}

#[test]
fn future_resolver_is_sync() {
    assert_sync::<FutureResolver>();
}

#[test]
fn c_ares_future_is_send() {
    assert_send::<CAresFuture<c_ares::AResults>>();
}

#[test]
fn c_ares_future_is_sync() {
    assert_sync::<CAresFuture<c_ares::AResults>>();
}

#[test]
fn error_is_send() {
    assert_send::<Error>();
}

#[test]
fn error_is_sync() {
    assert_sync::<Error>();
}

#[test]
fn host_results_is_send() {
    assert_send::<HostResults>();
}

#[test]
fn host_results_is_sync() {
    assert_sync::<HostResults>();
}

#[test]
fn name_info_result_is_send() {
    assert_send::<NameInfoResult>();
}

#[test]
fn name_info_result_is_sync() {
    assert_sync::<NameInfoResult>();
}
