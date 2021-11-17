pub mod preprocess {
    use crate::csv_read::csv_read;
    use std::string::String;
    use std::collections::HashMap;
    use std::collections::HashSet;

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
        let mut v = Vec::new();
        for r in vec {
            let data = DataRecord {
                question_title: split(r.question_title.to_lowercase()),
                question_text: split(r.question_text.to_lowercase()),
                topic: r.topic,
                answer_text: split(r.answer_text.to_lowercase()),
                upvotes: r.upvotes,
                views: r.views
            };
            v.push(data);
        }
        v
    }

    pub fn mapping(vec: Vec<DataRecord>) -> HashMap<String, i32> {
        let mut map: HashMap<String, i32> = HashMap::new();
        let mut set: HashSet<String> = HashSet::new();
        let mut i = 0;
        for record in vec {
            for q_title in record.question_title {
                set.insert(q_title);
            }
            for q_text in record.question_text {
                set.insert(q_text);
            }
            for a in record.answer_text {
                set.insert(a);
            }
        }
        for word in set {
            map.insert(word, i);
            i += 1;
        }
        map
    }

    fn split(s: String) -> Vec<String> {
        let mut vec: Vec<String> = Vec::new();
        let split_words = s.split(" ").to_owned();
        for word in split_words {
            vec.push(word.to_owned());
        }
        vec
    }
}