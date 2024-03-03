use crate::core::service::export_service::Provider;

pub struct SqlServerProvider {}

impl SqlServerProvider {
    pub fn new() -> Self {
        Self {}
    }
}

impl Provider for SqlServerProvider {
    fn open_connection(&mut self) -> anyhow::Result<()> {
        todo!()
    }

    fn send(&mut self, _: &str) -> anyhow::Result<()> {
        todo!()
    }
}
