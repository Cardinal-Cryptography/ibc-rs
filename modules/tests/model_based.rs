mod executor;

const TESTS_DIR: &str = "tests/support/model_based/tests";

#[test]
fn model_based() {
    let tests = vec![
        "ICS02UpdateOKTest",
        "ICS02HeaderVerificationFailureTest",
        "ICS03ConnectionOpenInitOKTest",
        "ICS03MissingClientTest",
        "ICS03ConnectionOpenTryOKTest",
        "ICS03InvalidConsensusHeightTest",
        "ICS03ConnectionNotFoundTest",
        "ICS03ConnectionMismatchTest",
    ];

    for test in tests {
        let test = format!("{}/{}.json", TESTS_DIR, test);
        println!("> running {}", test);
        let executor = executor::IBCTestExecutor::new();
        // we should be able to just return the `Result` once the following issue
        // is fixed: https://github.com/rust-lang/rust/issues/43301
        if let Err(e) = executor::modelator::test(&test, executor) {
            panic!("{}", e);
        }
    }
}
