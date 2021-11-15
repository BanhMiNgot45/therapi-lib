pub mod preprocess {
    use crate::csv_read::csv_read;
    use crate::text_read::text_read;
    use std::string::String;
    use std::collections::HashMap;

    pub struct DataRecord {
        pub question_title: Vec<String>,
        pub question_text: Vec<String>,
        pub topic: String,
        pub answer_text: Vec<String>,
        pub upvotes: i32,
        pub views: i32
    }

    pub fn preprocess() -> Vec<DataRecord> {
        let mut d = csv_read::Data::new("https://raw.githubusercontent.com/BanhMiNgot45/therapi-data/master/counsel_chat.csv".to_owned());
        d.read_file().unwrap();
        let vec = d.data;
        let stop = text_read::Data::new("https://raw.githubusercontent.com/BanhMiNgot45/therapi-data/master/stop_words.txt".to_owned());
        let punc = text_read::Data::new("https://raw.githubusercontent.com/BanhMiNgot45/therapi-data/master/punctuations.txt".to_owned());
        let mut v = Vec::new();
        for r in vec {
            let data = DataRecord {
                question_title: remove_noise(r.question_title.to_lowercase(), stop.data.clone(), &punc.data.clone()),
                question_text: remove_noise(r.question_text.to_lowercase(), stop.data.clone(), &punc.data.clone()),
                topic: r.topic,
                answer_text: remove_noise(r.answer_text.to_lowercase(), stop.data.clone(), &punc.data.clone()),
                upvotes: r.upvotes,
                views: r.views
            };
            v.push(data);
        }
        v
    }

    pub fn mapping(vec: Vec<String>) -> HashMap<String, i32> {
        let mut map: HashMap<String, i32> = HashMap::new();
        let mut i = 0;
        for s in vec {
            map.insert(s, i);
            i += 1;
        }
        map
    }

    fn remove_noise(s: String, stop: Vec<String>, punc: &Vec<String>) -> Vec<String> {
        let mut dummy: Vec<String> = Vec::new();
        let mut vec: Vec<String> = Vec::new();
        let split_words = s.split(" ").to_owned();
        for word in split_words {
            if !stop.contains(&word.to_owned()) {
                dummy.push(word.to_owned());
            }
        }
        for word in dummy {
            for p in punc {
                let w = word.replace(&p.to_owned(), "");
                vec.push(w);
            }
        }
        vec
    }
}