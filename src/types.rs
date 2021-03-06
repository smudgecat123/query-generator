use std::fmt;
use std::default;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Query {
    Add(Vec<String>, Vec<String>),
    Done(u64),
    Search(Vec<WordOrTag>),
}

impl fmt::Display for Query {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Query::Add(words, tags) => {
                let mut description = String::new();
                for word in words {
                    description.push_str(word);
                    description.push(' ');
                }
                description.pop();
                let mut tag_string = String::new();
                for tag in tags {
                    tag_string.push(' ');
                    tag_string.push('#');
                    tag_string.push_str(tag);
                }
                write!(f, "add \"{}\"{}", description, tag_string)
            },
            Query::Done(index) => {
                write!(f, "done {}", index)
            },
            Query::Search(params) => {
                let mut search_string = String::new();
                for param in params {
                    search_string.push(' ');
                    search_string.push_str(&param.to_string());
                }
                write!(f, "search{}", search_string)
            },
        }
    }
}

impl default::Default for Query {
    fn default() -> Self {
        Self::Done(0)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum WordOrTag {
    Word (String),
    Tag (String),
}

impl fmt::Display for WordOrTag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WordOrTag::Word(word) => {
                write!(f, "{}", word)
            },
            WordOrTag::Tag(tag) => {
                write!(f, "#{}", tag)
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn query_display_test() {
        let add_query = Query::Add(
            vec![
                "hello".to_string(),
                "world".to_string(),
            ], 
            vec![
                "these".to_string(),
                "are".to_string(),
                "the".to_string(),
                "tags".to_string(),
            ]
        );

        let done_query = Query::Done(4);

        let search_query = Query::Search(vec![
            WordOrTag::Word("hello".to_string()), 
            WordOrTag::Tag("world".to_string()),
            WordOrTag::Word("bello".to_string()),
            WordOrTag::Tag("burld".to_string())
        ]);

        assert_eq!(add_query.to_string(), "add \"hello world\" #these #are #the #tags".to_owned());
        assert_eq!(done_query.to_string(), "done 4".to_owned());
        assert_eq!(search_query.to_string(), "search hello #world bello #burld".to_owned());
    }
}