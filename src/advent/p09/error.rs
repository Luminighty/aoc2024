#[derive(Debug)]
#[allow(dead_code)]
pub enum AdventError {
	FileReadError,
	ParseIntError(String)
}

macro_rules! unwrap {
	($result: expr, $error: expr) => {{
		#[allow(unused_variables)]
		$result.map_err(|err| $error)
	}};
	($result: expr) => {{
		match $result {
			Ok(res) => res,
			Err(err) => {
				println!("Error: {:?}", err);
				return;
			}
		}
	}};
}
