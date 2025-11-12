use std::path::Path;
use tempfile::TempDir;

pub struct TestContext {
    tmp_dir: Option<TempDir>,
}

pub fn setup() -> TestContext {
    TestContext::new()
}

impl TestContext {
    pub fn new() -> Self {
        let tmp_dir = TempDir::new().unwrap();
        Self {
            tmp_dir: Some(tmp_dir),
        }
    }

    pub fn get_path(&self) -> &Path {
        self.tmp_dir.as_ref().unwrap().path()
    }
}

impl Drop for TestContext {
    fn drop(&mut self) {
        let tmp_dir = self.tmp_dir.take().unwrap();
        let path = String::from(tmp_dir.path().to_str().unwrap());
        tmp_dir
            .close()
            .expect(&format!("Could not close tmp dir in '{:?}'", path));
    }
}