// tests/node_smoke.rs

use assert_cmd::Command;
use std::io::Read;
use std::process::Stdio;
use std::time::Duration;
use tempfile::TempDir;
use wait_timeout::ChildExt;

/// Change this if your binary name differs
const NODE_BIN: &str = "btpc-quantum-resistant-chain";

fn help_or_version_works() {
    // Prefer --help (fast and always safe)
    let mut cmd = Command::cargo_bin(NODE_BIN).expect("binary not found");
    let out = cmd.arg("--help").output().expect("failed to run --help");
    if out.status.success() {
        let txt = String::from_utf8_lossy(&out.stdout);
        assert!(
            txt.contains("USAGE") || txt.contains("Usage") || !txt.is_empty(),
            "expected some help text"
        );
        return;
    }

    // Fallback to --version
    let mut cmd = Command::cargo_bin(NODE_BIN).expect("binary not found");
    let out = cmd
        .arg("--version")
        .output()
        .expect("failed to run --version");
    assert!(out.status.success(), "node --version failed");
}

#[test]
fn node_help_and_version() {
    help_or_version_works();
}

/// Try to actually start the node for a brief moment and make sure it doesn't crash immediately.
/// We don't assume any CLI flags; we just spawn it, wait a moment, and then kill.
#[test]
fn node_spawns_and_runs_briefly() {
    // Use a temp dir so we don't touch your real data.
    let data_dir = TempDir::new().expect("temp dir");

    // If your binary supports a --data-dir or similar flag, add it here.
    // We also pass an env var many projects use to reduce heavy work in tests.
    let mut child = Command::cargo_bin(NODE_BIN)
        .expect("binary not found")
        .env("BTPC_TEST", "1")
        .current_dir(data_dir.path())
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to spawn node");

    // Give it a short window to come up and print something.
    let timeout = Duration::from_secs(3);
    match child.wait_timeout(timeout).expect("wait failed") {
        // If it exited quickly, still allow it as long as exit was success and there was output.
        Some(status) => {
            assert!(status.success(), "node exited early with failure: {status}");
            let mut buf = String::new();
            if let Some(mut s) = child.stdout.take() {
                let _ = s.read_to_string(&mut buf);
            }
            let mut err = String::new();
            if let Some(mut e) = child.stderr.take() {
                let _ = e.read_to_string(&mut err);
            }
            assert!(
                !buf.is_empty() || !err.is_empty(),
                "expected some output from the node"
            );
        }
        // Still running — good! Proves it can start. Now stop it.
        None => {
            // Try a graceful kill (Unix: SIGTERM), else fall back to hard kill.
            #[cfg(unix)]
            {
                use nix::sys::signal::{kill, Signal};
                use nix::unistd::Pid;

                let pid = Pid::from_raw(child.id() as i32);
                // Ignore error here; we'll fall back to hard kill below if needed.
                let _ = kill(pid, Signal::SIGTERM);

                // Give it a moment to exit gracefully.
                let _ = child.wait_timeout(Duration::from_secs(1));
            }

            // Fallback to kill on all platforms (or if SIGTERM didn't exit it).
            let _ = child.kill();
            let _ = child.wait();
        }
    }
}
