use rand::seq::SliceRandom;
use crate::{data, stats};

#[derive(Debug, Clone, PartialEq)]
pub enum Screen {
    Menu,
    Drill,
    Result { correct: bool, expected: String, before: String, after: String },
    Summary,
    Heatmap,
    Streak,
}

pub struct Question {
    pub verb:   &'static str,
    pub tense:  &'static str,
    pub person: &'static str,
    pub before: &'static str,
    pub after:  &'static str,
}

pub struct App {
    pub screen:          Screen,
    pub questions:       Vec<Question>,
    pub q_index:         usize,
    pub input:           String,
    pub session_correct: u32,
    pub session_total:   u32,
    pub stats:           stats::Stats,
    pub tick:            u64,
}

impl App {
    pub fn new() -> Self {
        App {
            screen:          Screen::Menu,
            questions:       vec![],
            q_index:         0,
            input:           String::new(),
            session_correct: 0,
            session_total:   0,
            stats:           stats::load(),
            tick:            0,
        }
    }

    pub fn build_questions(&mut self) {
        let mut rng = rand::thread_rng();
        let mut qs: Vec<Question> = data::VERBS.iter().flat_map(|&v| {
            data::TENSES.iter().flat_map(move |&t| {
                data::PERSONS.iter().map(move |&p| {
                    let options = data::sentences(v, t, p);
                    let (before, after) = options[rand::random::<usize>() % options.len()];
                    Question { verb: v, tense: t, person: p, before, after }
                })
            })
        }).collect();
        qs.shuffle(&mut rng);
        self.questions       = qs;
        self.q_index         = 0;
        self.session_correct = 0;
        self.session_total   = 0;
        self.input.clear();
    }

    pub fn current_q(&self) -> Option<&Question> {
        self.questions.get(self.q_index)
    }

    pub fn submit_answer(&mut self) {
        if let Some(q) = self.questions.get(self.q_index) {
            let expected = data::conjugation(q.verb, q.tense, q.person);
            let correct  = self.input.trim().to_lowercase() == expected.to_lowercase();
            self.stats.record(q.verb, q.tense, q.person, correct);
            if correct { self.session_correct += 1; }
            self.session_total += 1;
            self.screen = Screen::Result {
                correct,
                expected: expected.to_string(),
                before:   q.before.to_string(),
                after:    q.after.to_string(),
            };
            self.input.clear();
        }
    }

    pub fn advance(&mut self) {
        self.q_index += 1;
        if self.q_index >= self.questions.len() {
            stats::save(&self.stats);
            self.screen = Screen::Summary;
        } else {
            self.screen = Screen::Drill;
        }
    }
}