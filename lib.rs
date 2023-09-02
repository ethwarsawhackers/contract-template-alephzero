#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod daQuiz {
    use ink::prelude::string::String;
    use ink::prelude::vec::Vec;
    use ink::storage::traits::StorageLayout;

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, StorageLayout))]
    pub enum Error {
        NotAllowed, NotFound
    }

    #[derive(Debug, Default, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, StorageLayout))]
    pub struct Style {
        pub background: String,
        pub text: String,
    }

    #[derive(Debug, Default, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, StorageLayout))]
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
    
    #[derive(Debug, Default, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, StorageLayout))]
    pub struct QuestionEntry {
        pub id: String,
        pub answers: Vec<String>
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, StorageLayout))]
    pub struct InputEntry {
        pub author: String,
        pub questions: Vec<QuestionEntry>
    }

    #[derive(Debug, PartialEq, Clone, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, StorageLayout))]
    pub struct Entry {
        pub author: AccountId,
        pub questions: Vec<QuestionEntry>
    }
    
    #[derive(Debug, Clone, Default, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, StorageLayout))]
    pub struct Answer {
        pub caption: String,
    }

    #[derive(Debug, Default, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, StorageLayout))]
    pub struct Question {
        pub id: String,
        pub question: String,
        pub note: String,
        pub answers: Vec<Answer>,
    }

    #[derive(Debug, Default, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, StorageLayout))]
    pub struct DaState {
        pub metadata: Metadata,
        pub entries: Vec<Entry>,
        pub questions: Vec<Question>
    }

    #[ink(storage)]
    pub struct DaQuiz {
        operator: AccountId,
        state: DaState
    }

    impl DaQuiz {
        #[ink(constructor)]
        pub fn new(init_metadata: Metadata, init_questions: Vec<Question>) -> Self {
            Self {
                operator: Self::env().caller(),
                state: DaState {
                    metadata: init_metadata, 
                    entries: Vec::new(), 
                    questions: init_questions
                }
            }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default(), Default::default())
        }

        #[ink(message)]
        pub fn getMetadata(&self) -> Metadata {
            let meta = &self.state.metadata;
            meta.clone()
        }

        fn filter<T: Clone>(indices: Vec<u32>, questions: Vec<T>) -> Vec<T> {
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
            Self::filter(indices, self.state.questions.clone())
        }
        
        #[ink(message)]
        pub fn getEntries(&self) -> Vec<Entry> {
            assert_eq!(
                self.env().caller(),
                self.operator,
                "caller {:?} does not have sufficient permissions, only {:?} does",
                self.env().caller(),
                self.operator,
            );
            
            let result = &self.state.entries;
            result.clone().to_vec()
        }
        
        #[ink(message)]
        pub fn setOwnEntry(&mut self, data: InputEntry) -> Result<(), Error> {
            if self.state.metadata.maxEntries > self.state.entries.len().try_into().unwrap() {
            
                let bytes = data.author.as_bytes();
                let id = AccountId::try_from(bytes)
                    .expect("Incorrect address");
        
                self.state.entries.push(Entry {
                    author: id,
                    questions: data.questions
                });

                Ok(())
            } else {
                Err(Error::NotAllowed)
            }
        }
        
        #[ink(message)]
        pub fn updateMetadata(&mut self, data: Metadata) -> Result<(), Error> {
            if self.env().caller().eq(&self.operator) { 
                self.state.metadata = data;
                Ok(())
            } else {
                Err(Error::NotAllowed)
            }
        }

        #[ink(message)]
        pub fn updateQuestion(&mut self, question: Question) -> Result<(), Error> {
            if self.env().caller().eq(&self.operator) { 
                if let Some(pos) = self.state.questions.iter().position(|q| q.id == question.id) {
                    self.state.questions[pos] = question;
                    Ok(())
                } else {
                    self.state.questions.push(question);
                    Ok(())
                }
            } else {
                Err(Error::NotAllowed)
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
            assert_eq!(daQuiz.getMetadata(), metadata.clone());
        }
    }
}
