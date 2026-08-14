use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
use crate::diagnostic::Diagnostic;
use crate::japanese::Translator;
use crate::renderer::TerminalRenderer;

pub struct RustcRunner {
    #[allow(dead_code)]
    pub translator: Translator,
    pub renderer: TerminalRenderer,
}

impl RustcRunner {
    pub fn new(show_original: bool, quiet: bool, verbose: bool) -> Self {
        Self {
            translator: Translator::new(),
            renderer: TerminalRenderer::new(show_original, quiet, verbose),
        }
    }

    pub fn compile_file(&self, file_path: &str, extra_args: &[String]) -> Result<i32, Box<dyn std::error::Error>> {
        let mut cmd = Command::new("rustc");
        cmd.arg(file_path);
        cmd.arg("--error-format=json");

        for arg in extra_args {
            cmd.arg(arg);
        }

        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());

        let mut child = cmd.spawn().map_err(|e| {
            format!("rustcの起動に失敗しました: {}", e)
        })?;

        let stderr = child.stderr.take().unwrap();
        let translator = Translator::new();
        let renderer = TerminalRenderer::new(
            self.renderer.show_original,
            self.renderer.quiet,
            self.renderer.verbose,
        );

        let stderr_handle = std::thread::spawn(move || {
            let reader = BufReader::new(stderr);
            let mut collected = Vec::new();
            for line_res in reader.lines() {
                if let Ok(line) = line_res {
                    if let Ok(diag) = serde_json::from_str::<Diagnostic>(&line) {
                        let jd = translator.translate(&diag);
                        renderer.render(&jd);
                        collected.push(jd);
                    } else if !line.trim().is_empty() {
                        eprintln!("{}", line);
                    }
                }
            }
            if !collected.is_empty() {
                renderer.render_summary_table(&collected);
            }
        });

        let _ = stderr_handle.join();
        let status = child.wait()?;
        Ok(status.code().unwrap_or(1))
    }
}
