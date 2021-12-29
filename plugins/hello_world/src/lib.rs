pub struct HelloWorldPlugin {}

impl indradb_plugin_host::Plugin for HelloWorldPlugin {
    fn call(
        &self,
        _datastore: Box<dyn indradb::Transaction + Send>,
        arg: serde_json::Value,
    ) -> Result<serde_json::Value, indradb::Error> {
        let greeting = format!("hello, {}", arg);
        Ok(serde_json::Value::String(greeting))
    }
}

indradb_plugin_host::register_plugins!(0, "hello_world", Box::new(crate::HelloWorldPlugin {}));