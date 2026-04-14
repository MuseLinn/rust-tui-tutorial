use crate::models::CompileResult;
use tokio::process::Command;
use tokio::time::{timeout, Duration};

pub fn compile_and_run(code: &str, expected_output: &str) -> CompileResult {
    // Since this may be called from sync context, we use tokio::runtime::Handle::block_on
    // or the caller can use a spawned task. For safety, we'll implement the async part
    // and provide a blocking wrapper.
    let rt = match tokio::runtime::Handle::try_current() {
        Ok(h) => h,
        Err(_) => {
            // No runtime available — return a generic error
            return CompileResult::CompileError {
                stderr: "Validator must run inside a Tokio runtime".into(),
            };
        }
    };

    match rt.block_on(compile_and_run_async(code, expected_output)) {
        Ok(result) => result,
        Err(_) => CompileResult::Timeout,
    }
}

async fn compile_and_run_async(code: &str, expected_output: &str) -> std::io::Result<CompileResult> {
    let tmp = tempfile::tempdir()?;
    let src_path = tmp.path().join("main.rs");
    tokio::fs::write(&src_path, code).await?;

    let out_path = tmp.path().join("main");

    let compile_result = timeout(
        Duration::from_secs(30),
        Command::new("rustc")
            .arg("--edition=2021")
            .arg("-o")
            .arg(&out_path)
            .arg(&src_path)
            .output(),
    )
    .await;

    match compile_result {
        Ok(Ok(output)) if output.status.success() => {
            // Compiled successfully — run the binary
            let run_result = timeout(
                Duration::from_secs(5),
                Command::new(&out_path).output(),
            )
            .await;

            match run_result {
                Ok(Ok(run_output)) if run_output.status.success() => {
                    let stdout = String::from_utf8_lossy(&run_output.stdout).trim().to_string();
                    let expected = expected_output.trim().to_string();
                    if stdout == expected {
                        Ok(CompileResult::Success)
                    } else {
                        Ok(CompileResult::RuntimeMismatch {
                            expected,
                            got: stdout,
                        })
                    }
                }
                Ok(Ok(run_output)) => {
                    let stderr = String::from_utf8_lossy(&run_output.stderr).to_string();
                    Ok(CompileResult::CompileError { stderr })
                }
                Ok(Err(e)) => Err(e),
                Err(_) => Ok(CompileResult::Timeout),
            }
        }
        Ok(Ok(output)) => {
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            Ok(CompileResult::CompileError { stderr })
        }
        Ok(Err(e)) => Err(e),
        Err(_) => Ok(CompileResult::Timeout),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_compile_success() {
        let code = r#"fn main() { println!("Hello"); }"#;
        let result = compile_and_run_async(code, "Hello").await.unwrap();
        assert_eq!(result, CompileResult::Success);
    }

    #[tokio::test]
    async fn test_compile_error() {
        let code = r#"fn main() { println!("Hello" }"#;
        let result = compile_and_run_async(code, "Hello").await.unwrap();
        match result {
            CompileResult::CompileError { .. } => {}
            _ => panic!("expected compile error, got {:?}", result),
        }
    }

    #[tokio::test]
    async fn test_runtime_mismatch() {
        let code = r#"fn main() { println!("World"); }"#;
        let result = compile_and_run_async(code, "Hello").await.unwrap();
        match result {
            CompileResult::RuntimeMismatch { .. } => {}
            _ => panic!("expected runtime mismatch, got {:?}", result),
        }
    }
}
