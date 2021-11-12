pub mod csv_read {
    use std::error::Error;
    use std::io::copy;
    use std::fs::File;
    use tempfile::Builder;
    use serde::Deserialize;
    use std::io::Read;
    use std::string::String;

    #[derive(Deserialize)]
    pub struct Record {
        record_id: i32,
        question_id: i32,
        pub question_title: String,
        pub question_text: String,
        question_link: String,
        pub topic: String,
        therapist_info: String,
        therapist_url: String,
        pub answer_text: String,
        pub upvotes: i32,
        pub views: i32,
        split: String
    }
    
    pub struct Data {
        url: String,
        pub data: Vec<Record>
    }

    impl Data {
        pub fn new(url: String) -> Data {
            Data {
                url: url,
                data: Vec::new()
            }
        }

        #[tokio::main]
        pub async fn read_file(&mut self) -> Result<(), Box<dyn Error>> {
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
            let mut csv = csv::Reader::from_reader(buffer.as_bytes());
            for record in csv.deserialize() {
                let r = record?;
                self.data.push(r);
            }
            Ok(())
        }
    }
}