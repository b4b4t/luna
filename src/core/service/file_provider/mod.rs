use super::export_service::Provider;
use std::{fs::File, io::Write};

pub struct FileProvider {
    file_name: String,
    file: Option<File>,
}

impl FileProvider {
    pub fn new(file_name: &str) -> Self {
        Self {
            file_name: file_name.to_string(),
            file: None,
        }
    }
}

impl Provider for FileProvider {
    fn send(&mut self, data: &str) -> anyhow::Result<()> {
        let file = self.file.as_mut().expect("File cannot be written");
        writeln!(file, "{}", data)?;

        Ok(())
    }

    fn open_connection(&mut self) -> anyhow::Result<()> {
        let file = File::create(&self.file_name).expect("Cannot create the file");
        self.file = Some(file);

        Ok(())
    }
}
