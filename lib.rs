#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod daQuiz {
    use ink::prelude::string::String;
    use ink::prelude::vec::Vec;

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        NotAnOwner, NotAllowed, NotFound
    }

    #[derive(Debug, Default, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct Style {
        pub background: String,
        pub text: String,
    }

    #[derive(Debug, Default, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct Metadata {
        pub title: String,
        pub tokens: String,
        pub ticker: String,
        pub chain: String,
        pub allow: Vec<String>,
        pub maxEntries: u64,
        pub note: String,
        pub style: Style,
    }
    
    #[derive(Debug, Default, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct QuestionEntry {
        pub id: String,
        pub answers: Vec<String>
    }

    #[derive(Debug, Default, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct Entry {
        pub author: String,
        pub questions: Vec<QuestionEntry>
    }
    
    #[derive(Debug, Clone, Default, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct Answer {
        pub caption: String,
    }

    #[derive(Debug, Clone, Default, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct Question {
        pub id: String,
        pub question: String,
        pub note: String,
        pub answers: Vec<Answer>,
    }

    #[ink(storage)]
    #[derive(Default)]
    pub struct DaQuiz {
        pub metadata: Metadata,
        pub entries: Vec<Entry>,
        pub questions: Vec<Question>
    }

    impl DaQuiz {
        #[ink(constructor)]
        pub fn new(init_metadata: Metadata, init_questions: Vec<Question>) -> Self {
            Self {
                metadata: init_metadata, 
                entries: Vec::new(), 
                questions: init_questions 
            }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default(), Default::default())
        }

        #[ink(message)]
        pub fn getMetadata(&self) -> Metadata {
            self.metadata
        }

        fn filter_by_indices<T: Clone>(indices: Vec<u32>, questions: Vec<T>) -> Vec<T> {
            let mut filtered_questions = Vec::new();
            for &index in &indices {
                if index < questions.len() as u32 {
                    filtered_questions.push(questions[index as usize].clone());
                }
            }
            filtered_questions
        }        

        #[ink(message)]
        pub fn getQuestions(&self, indices: Vec<u32>) -> Vec<Question> {
            Self::filter_by_indices(indices, self.questions)
        }
        
        #[ink(message)]
        pub fn getEntries(&self) -> Vec<Entry> {
            self.entries
        }
        
        #[ink(message)]
        pub fn setOwnEntry(&mut self, data: Entry) {
            self.entries.push(data)
        }
        
        #[ink(message)]
        pub fn updateMetadata(&mut self, data: Metadata) {
            self.metadata = data
        }

        #[ink(message)]
        pub fn updateQuestion(&mut self, question: Question) -> Result<(), Error> {
            if let Some(pos) = self.questions.iter().position(|&q| q.id == question.id) {
                self.questions[pos] = question;
                Ok(())
            } else {
                return Err(Error::NotFound)
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn default_works() {
            let daQuiz = DaQuiz::default();
            assert_eq!(daQuiz.getEntries(), vec![] as Vec<Entry>);
        }

        #[ink::test]
        fn it_allows_submissions() {
            let metadata = Metadata {
                title: "test-title".to_string(),
                tokens: "100200300400500600700800900".to_string(),
                chain: "Ethereum".to_string(),
                ticker: "ETH".to_string(),
                allow: vec![],
                maxEntries: 100,
                note: Default::default(),
                style: Default::default()   
            };

            let mut daQuiz = DaQuiz::new(metadata, Default::default());
            assert_eq!(daQuiz.getMetadata(), metadata);
        }
    }
}
