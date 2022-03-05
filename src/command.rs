pub struct Command {
    pub command_line: String,
    callback: Box<dyn FnMut(Result<String, String>)>
}

impl Command {
    pub fn from(command_line: String, callback: impl FnMut(Result<String, String>) + Send + Sync + 'static) -> Command {
        Command {
            command_line: command_line.trim().to_owned(),
            callback: Box::new(callback)
        }
    }

    pub fn handle_result(&mut self, result: Result<String, String>) {
        (self.callback)(result);
    }
}

unsafe impl Send for Command {}
unsafe impl Sync for Command {}