const CURSOR_POSITION_QUERY: &str = "\x1b[6n";
const CURSOR_POSITION_REPLY: &str = "\x1b[1;1R";
const DEVICE_ATTRIBUTES_QUERY: &str = "\x1b[c";
const DEVICE_ATTRIBUTES_REPLY: &str = "\x1b[?1;2c";

/// ConPTY sends its startup queries before anything else, later queries belong to the frontend terminal
const STARTUP_WINDOW_BYTES: usize = 256;

/*	NOTE: ConPTY asks the terminal for the cursor position and the device attributes while it starts.
	Without the cursor position the ConPTY built into Windows never starts the shell, without the device attributes
	the shipped ConPTY starts the shell one second late. Shells are spawned without a frontend terminal attached,
	so nobody would answer. The queries are answered here and removed from the output, otherwise the frontend
	terminal would answer them again when the history is restored.
*/
pub struct ConptyStartupQueries {
	cursor_position_pending: bool,
	device_attributes_pending: bool,
	bytes_seen: usize,
}

impl ConptyStartupQueries {
	pub fn new() -> Self {
		Self {
			cursor_position_pending: true,
			device_attributes_pending: true,
			bytes_seen: 0,
		}
	}

	/// Removes the startup queries from the output and returns the reply that has to be written to the pty
	pub fn answer(&mut self, output: &mut String) -> Option<String> {
		if self.bytes_seen >= STARTUP_WINDOW_BYTES {
			return None;
		}

		self.bytes_seen += output.len();

		let mut reply = String::new();

		if self.cursor_position_pending && Self::remove(output, CURSOR_POSITION_QUERY) {
			self.cursor_position_pending = false;
			reply.push_str(CURSOR_POSITION_REPLY);
		}

		if self.device_attributes_pending && Self::remove(output, DEVICE_ATTRIBUTES_QUERY) {
			self.device_attributes_pending = false;
			reply.push_str(DEVICE_ATTRIBUTES_REPLY);
		}

		if reply.is_empty() {
			None
		} else {
			Some(reply)
		}
	}

	fn remove(output: &mut String, query: &str) -> bool {
		match output.find(query) {
			Some(start) => {
				output.replace_range(start..start + query.len(), "");
				true
			}
			None => false,
		}
	}
}
