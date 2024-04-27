use assert_cmd::Command as TestingCommand;
use std::process::Command;

/// Just make sure that rusty_wc behaves the same as wc.
#[test]
fn test_wc_compatibility() {
    let wc_compat_test_cases: Vec<Vec<&str>> = vec![
        vec!["-l", "LICENSE", "CONTRIBUTING.md"],
        vec!["-l", "LICENSE"],
        vec!["-w", "LICENSE", "CONTRIBUTING.md"],
        vec!["-w", "LICENSE"],
        vec!["-c", "LICENSE", "CONTRIBUTING.md"],
        vec!["-c", "LICENSE"],
        vec!["LICENSE", "CONTRIBUTING.md"],
        vec!["LICENSE"],
    ];

    for wc_args in wc_compat_test_cases {
        // First, just run wc and save the output
        let legit_wc_output = Command::new("wc")
            .args(wc_args.clone())
            .output()
            .expect("Failed to run wc");

        let mut cmd = TestingCommand::cargo_bin("rusty-wc").unwrap();
        let rusty_wc_output = cmd.args(wc_args).output().expect("Failed to run rusty-wc");
        assert_eq!(legit_wc_output, rusty_wc_output);
    }
}
