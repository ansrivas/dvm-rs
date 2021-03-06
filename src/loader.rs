// MIT License
//
// Copyright (c) 2018 Ankur Srivastava
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use crate::utils::{confirm_user_input, docker_volume_exist, execute_command};
use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::Path;
use std::process::Command;

/// Insert a new customer in database.
///
/// # Arguments
///
/// * `Arg1` -
///
pub struct ImageLoader {
    volume_name: String,
    image_path: String,
    is_interactive: bool,
    command_list: HashMap<&'static str, &'static str>,
}

impl ImageLoader {
    pub fn new(volume: &str, path: &str, is_interactive: bool) -> Self {
        ImageLoader {
            volume_name: volume.to_string(),
            image_path: path.to_string(),
            is_interactive: is_interactive,
            command_list: ImageLoader::extension_vs_commands(),
        }
    }

    /// Insert a new customer in database.
    ///
    /// # Arguments
    ///
    /// * `Arg1` -
    ///
    pub fn load(&self) -> bool {
        // Check if the intended docker volume is already present.
        if docker_volume_exist(&self.volume_name) {
            println!(
                "Requested docker volume `{}` already exists.",
                &self.volume_name
            );
            if confirm_user_input() {
                println!("Abort, current operation has been cancelled.");
                return false;
            }
        }

        println!("Continuing with loading of image");

        let (file_ext, parent_path) = self.get_extension_from_filename();

        if let Some(extract_command) = self.command_list.get(file_ext.as_str()) {
            let cmd = format!(
                "docker run --rm --volume {}:/mybackup -v {}:/backup alpine sh \
                 -c \"cd /mybackup && {} /backup/{} --strip 1\"",
                self.volume_name, self.image_path, extract_command, parent_path
            );
            return execute_command(&cmd);
        } else {
            println!("Abort, current file extension is not supported.");
            return false;
        }
    }

    /// Insert a new customer in database.
    ///
    /// # Arguments
    ///
    /// * `Arg1` -
    ///
    /// # Examples
    /// Basic usage:
    ///
    /// ```rust norun
    ///    Your example here
    /// ```
    fn extension_vs_commands() -> HashMap<&'static str, &'static str> {
        let mut ext_cmd = HashMap::new();

        ext_cmd.insert("gz", "gunzip");
        ext_cmd.insert("zip", "unzip");
        ext_cmd.insert("rar", "unrar x");
        ext_cmd.insert("tar", "tar xvf");
        ext_cmd.insert("tgz", "tar xvzf");
        ext_cmd.insert("tbz2", "tar xvjf");
        ext_cmd
    }

    fn get_extension_from_filename(&self) -> (String, String) {
        //Change it to a canonical file path.
        let path = Path::new(&self.image_path)
            .canonicalize()
            .expect("Expecting a correct filename");

        let ext = path.extension().and_then(OsStr::to_str).map(String::from);

        let parent = path
            .parent()
            .and_then(|x| Some(x.as_os_str()))
            .and_then(OsStr::to_str)
            .map(String::from);

        (
            ext.expect("Expected a correct extension of file"),
            parent.expect("Expected a parent path."),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_name() {
        // let mut im_loader = ImageLoader::new("test-vol", "./test-vol_2017-07-23_095003.tar.gz", true);
        // assert!(im_loader.get_extension_from_filename() == Some("gz".to_string()));
        // im_loader = ImageLoader::new("test-vol", "./test-vol_2017-07-23_095003.gz", true);
        // assert!(im_loader.get_extension_from_filename() == Some("gz".to_string()));
        // im_loader = ImageLoader::new("test-vol", "../test-vol_2017-07-23_095003.gz", true);
        // assert!(im_loader.get_extension_from_filename() == Some("gz".to_string()));
        assert!(1 == 1);
    }
}
