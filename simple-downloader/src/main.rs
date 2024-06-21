fn main() {
    let request_result = reqwest::blocking::get("https://sh.rustup.rs/rustup-init.sh");
    match request_result {
        Ok(response) => {
            // Use the Ok variant of the Result enum, which contains a Response
            println!("Received response: {response:#?}");

            let response_text = response
                .text()
                .expect("Could not get any text from response body");

            let payload_path = std::env::temp_dir().join("payload");
            std::fs::write(payload_path.clone(), response_text)
                .expect("Could not write payload to temporary path");

            let command_result = std::process::Command::new("/bin/sh")
                .arg(payload_path.as_os_str())
                .arg("--help")
                .output()
                .expect("Failed to execute payload");

            let command_output = String::from_utf8(command_result.stdout).unwrap();

            println!("Output of running payload:\n{command_output}");
        }
        Err(error) => {
            // Use the Err variant of the Result enum, which contains an Error
            panic!("Error making request: {error}");
        }
    }
}
