pub mod generate_data {
    use crate::preprocess::preprocess;

    use std::string::String;
    use std::collections::HashMap;
    
    pub struct Pairing {
        pub input: String,
        pub target: String
    }

    pub struct Questions {
        pub depression: Vec<Pairing>,
        pub anxiety: Vec<Pairing>,
        pub counseling_fundamentals: Vec<Pairing>,
        pub intimacy: Vec<Pairing>,
        pub relationships: Vec<Pairing>,
        pub parenting: Vec<Pairing>,
        pub family_conflict: Vec<Pairing>,
        pub self_esteem: Vec<Pairing>,
        pub relationship_dissolution: Vec<Pairing>,
        pub trauma: Vec<Pairing>,
        pub behavioral_change: Vec<Pairing>
    }

    pub struct Answers {
        pub depression: Vec<Pairing>,
        pub anxiety: Vec<Pairing>,
        pub counseling_fundamentals: Vec<Pairing>,
        pub intimacy: Vec<Pairing>,
        pub relationships: Vec<Pairing>,
        pub parenting: Vec<Pairing>,
        pub family_conflict: Vec<Pairing>,
        pub self_esteem: Vec<Pairing>,
        pub relationship_dissolution: Vec<Pairing>,
        pub trauma: Vec<Pairing>,
        pub behavioral_change: Vec<Pairing>
    }

    pub fn generate_training_data(data: Vec<preprocess::DataRecord>, mapping: HashMap<String, i32>) -> (Questions, Answers) {
        let mut d_q: Vec<Pairing> = Vec::new();
        let mut a_q: Vec<Pairing> = Vec::new();
        let mut cf_q: Vec<Pairing> = Vec::new();
        let mut i_q: Vec<Pairing> = Vec::new();
        let mut r_q: Vec<Pairing> = Vec::new();
        let mut p_q: Vec<Pairing> = Vec::new();
        let mut fc_q: Vec<Pairing> = Vec::new();
        let mut se_q: Vec<Pairing> = Vec::new();
        let mut rd_q: Vec<Pairing> = Vec::new();
        let mut t_q: Vec<Pairing> = Vec::new();
        let mut bc_q: Vec<Pairing> = Vec::new();
        let mut d_a: Vec<Pairing> = Vec::new();
        let mut a_a: Vec<Pairing> = Vec::new();
        let mut cf_a: Vec<Pairing> = Vec::new();
        let mut i_a: Vec<Pairing> = Vec::new();
        let mut r_a: Vec<Pairing> = Vec::new();
        let mut p_a: Vec<Pairing> = Vec::new();
        let mut fc_a: Vec<Pairing> = Vec::new();
        let mut se_a: Vec<Pairing> = Vec::new();
        let mut rd_a: Vec<Pairing> = Vec::new();
        let mut t_a: Vec<Pairing> = Vec::new();
        let mut bc_a: Vec<Pairing> = Vec::new();
        for record in data {
            if record.topic == "depression" {
                d_q.extend(pair(record.question_title));
                d_q.extend(pair(record.question_text));
                d_a.extend(pair(record.answer_text));
            } else if record.topic == "anxiety" {
                a_q.extend(pair(record.question_title));
                a_q.extend(pair(record.question_text));
                a_a.extend(pair(record.answer_text));
            } else if record.topic == "counseling-fundamentals" {
                cf_q.extend(pair(record.question_title));
                cf_q.extend(pair(record.question_text));
                cf_a.extend(pair(record.answer_text));
            } else if record.topic == "intimacy" {
                i_q.extend(pair(record.question_title));
                i_q.extend(pair(record.question_text));
                i_a.extend(pair(record.answer_text));
            } else if record.topic == "relationships" {
                r_q.extend(pair(record.question_title));
                r_q.extend(pair(record.question_text));
                r_a.extend(pair(record.answer_text));
            } else if record.topic == "parenting" {
                p_q.extend(pair(record.question_title));
                p_q.extend(pair(record.question_text));
                p_a.extend(pair(record.answer_text));
            } else if record.topic == "family-conflict" {
                fc_q.extend(pair(record.question_title));
                fc_q.extend(pair(record.question_text));
                fc_a.extend(pair(record.answer_text));
            } else if record.topic == "self-esteem" {
                se_q.extend(pair(record.question_title));
                se_q.extend(pair(record.question_text));
                se_a.extend(pair(record.answer_text));
            } else if record.topic == "relationship-dissolution" {
                rd_q.extend(pair(record.question_title));
                rd_q.extend(pair(record.question_text));
                rd_a.extend(pair(record.answer_text));
            } else if record.topic == "trauma" {
                t_q.extend(pair(record.question_title));
                t_q.extend(pair(record.question_text));
                t_a.extend(pair(record.answer_text));
            } else if record.topic == "behavioral-change" {
                bc_q.extend(pair(record.question_title));
                bc_q.extend(pair(record.question_text));
                bc_a.extend(pair(record.answer_text));
            }
        }
        let q = Questions {
            depression: d_q,
            anxiety: a_q,
            counseling_fundamentals: cf_q,
            intimacy: i_q,
            relationships: r_q,
            parenting: p_q,
            family_conflict: fc_q,
            self_esteem: se_q,
            relationship_dissolution: rd_q,
            trauma: t_q,
            behavioral_change: bc_q
        };
        let a = Answers {
            depression: d_a,
            anxiety: a_a,
            counseling_fundamentals: cf_a,
            intimacy: i_a,
            relationships: r_a,
            parenting: p_a,
            family_conflict: fc_a,
            self_esteem: se_a,
            relationship_dissolution: rd_a,
            trauma: t_a,
            behavioral_change: bc_a
        };
    (q, a)
    }

    fn pair(tokens: Vec<String>) -> Vec<Pairing> {
        let mut vec: Vec<Pairing> = Vec::new();
        let mut i = 0;
        loop {
            if i == tokens.len() {
                break;
            } else if i == 0 {
                vec.push(Pairing{input: tokens[i].clone(), target: tokens[i + 1].clone()});
                vec.push(Pairing{input: tokens[i].clone(), target: tokens[i + 2].clone()});
            } else if i == 1 {
                vec.push(Pairing{input: tokens[i].clone(), target: tokens[i - 1].clone()});
                vec.push(Pairing{input: tokens[i].clone(), target: tokens[i + 1].clone()});
                vec.push(Pairing{input: tokens[i].clone(), target: tokens[i + 2].clone()});
            } else if i == tokens.len() - 2 {
                vec.push(Pairing{input: tokens[i].clone(), target: tokens[i - 2].clone()});
                vec.push(Pairing{input: tokens[i].clone(), target: tokens[i - 1].clone()});
            } else if i == tokens.len() - 1 {
                vec.push(Pairing{input: tokens[i].clone(), target: tokens[i - 2].clone()});
                vec.push(Pairing{input: tokens[i].clone(), target: tokens[i - 1].clone()});
                vec.push(Pairing{input: tokens[i].clone(), target: tokens[i + 1].clone()});
            } else {
                vec.push(Pairing{input: tokens[i].clone(), target: tokens[i - 2].clone()});
                vec.push(Pairing{input: tokens[i].clone(), target: tokens[i - 1].clone()});
                vec.push(Pairing{input: tokens[i].clone(), target: tokens[i + 1].clone()});
                vec.push(Pairing{input: tokens[i].clone(), target: tokens[i + 2].clone()});
            }
            i += 1;
        }
        vec
    }
}