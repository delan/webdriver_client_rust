//! Support for the Chrome browser.

use super::*;

use std::process::{Command, Child, Stdio};
use std::thread;
use std::time::Duration;
use std::ffi::OsString;

use super::util;

pub struct ChromeDriverBuilder {
    driver_binary: OsString,
    port: Option<u16>,
    kill_on_drop: bool,
}

impl ChromeDriverBuilder {
    pub fn new<S: Into<OsString>>(path: S) -> Self {
        ChromeDriverBuilder {
            driver_binary: path.into(),
            port: None,
            kill_on_drop: true,
        }
    }
    pub fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }
    pub fn kill_on_drop(mut self, kill: bool) -> Self {
        self.kill_on_drop = kill;
        self
    }
    pub fn spawn(self) -> Result<ChromeDriver, Error> {
        let port = util::check_tcp_port(self.port)?;

        let child = Command::new(self.driver_binary)
            .arg(format!("--port={}", port))
            .stdin(Stdio::null())
            .stderr(Stdio::null())
            .stdout(Stdio::null())
            .spawn()?;

        // TODO: parameterize this
        thread::sleep(Duration::new(1, 500));
        Ok(ChromeDriver {
            child: child,
            url: format!("http://localhost:{}", port),
            kill_on_drop: self.kill_on_drop,
        })
    }
}

/// The `default()` driver expects `chromedriver` in your `$PATH`
impl Default for ChromeDriverBuilder {
    fn default() -> Self {
        Self::new("chromedriver")
    }
}

/// A chromedriver process
pub struct ChromeDriver {
    child: Child,
    url: String,
    kill_on_drop: bool,
}

impl ChromeDriver {
    pub fn spawn() -> Result<Self, Error> {
        ChromeDriverBuilder::default().spawn()
    }
    pub fn build() -> ChromeDriverBuilder {
        ChromeDriverBuilder::default()
    }
}

impl Drop for ChromeDriver {
    fn drop(&mut self) {
        if self.kill_on_drop {
            let _ = self.child.kill();
        }
    }
}

impl Driver for ChromeDriver {
    fn url(&self) -> &str {
        &self.url
    }
}
