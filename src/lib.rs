
pub mod file
{
	use std::fs;
	pub fn read(filepath: String) -> String
	{
		let content: String = fs::read_to_string(filepath)
			.expect("file::read - error: cannot read file");
		return content;
	}
}





#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_file_read()
	{
		let result = file::read(String::from("res/fileread.test"));
		let expect = "Parrot is the best!";
		assert_eq!(result, expect);
	}
}
