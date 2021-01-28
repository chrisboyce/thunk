#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug)]
struct Note {
    content: Vec<Content>,
}

impl Default for Note {
    fn default() -> Self {
        Self { content: vec![] }
    }
}

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug)]
enum Media {
    Image,
}

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug)]
enum Image {
    Attached,
    Linked,
}

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug)]
enum Link {
    Parent,
    Association,
}

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug)]
enum Content {
    Media(Media),
    Text(String),
    Tag(String),
    Identifier(Identifier),
    Link(String),
    Image,
    Note(Box<Note>),
}


#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug)]
struct Child(Content);

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug)]
enum Identifier {
    Person(Person),
    Place,
    Time,
    Thing,
}

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug)]
struct Person {
    name: String,
}

//
#[cfg(test)]
mod test {
    use crate::note::{Content, Identifier, Note, Person};

    #[test]
    fn test_note() {
        let n = Note {
            content: vec![
                Content::Identifier(Identifier::Time),
                Content::Note(Box::new(Note {
                    content: vec![Content::Tag(String::from("ML"))],
                })),
                Content::Text(String::from("Call with guy at")),
                Content::Tag(String::from("ISG")),
                Content::Identifier(Identifier::Person(Person {
                    name: "Peter Graham".to_string(),
                })),
            ],
        };
        println!("{:#?}", n);
    }
}
