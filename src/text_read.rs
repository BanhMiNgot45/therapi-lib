pub mod text_read {
    use std::string::String;
    use std::error::Error;
    use tempfile::Builder;
    use std::fs::File;
    use std::io::copy;
    use std::io::Read;

    struct Data {
        url: String,
        data: Vec<String>
    }

    impl Data {
        fn new(url: String) -> Data {
            Data {
                url: url,
                data: Vec::new()
            }
        }

        async fn read_file(&mut self) -> Result<(), Box<dyn Error>> {
            let tmp_dir = Builder::new().prefix("tmp").tempdir()?;
            let response = reqwest::get(self.url.to_string()).await?;
            let mut dest = {
                let fname = response
                    .url()
                    .path_segments()
                    .and_then(|segments| segments.last())
                    .and_then(|name| if name.is_empty() { None } else { Some(name) })
                    .unwrap_or("tmp.bin");
                let fname = tmp_dir.path().join(fname);
                File::create(fname)?
            };
            let content = response.text().await?;
            copy(&mut content.as_bytes(), &mut dest)?;
            let mut buffer = String::new();
            dest.read_to_string(&mut buffer)?;
            let split_terminator = buffer.split_terminator("\n").to_owned();
            for s in split_terminator {
                self.data.push(s.to_owned());
            }
            Ok(())
        }
    }
}