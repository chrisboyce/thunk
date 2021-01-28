/// A `Note` is meant to be a sentence or short paragraph. It serves
/// to associate series of `Content`s together.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug)]
struct Note {
    content: Vec<Content>,
    children: Vec<Note>,
}

impl Default for Note {
    fn default() -> Self {
        Self {
            content: vec![],
            children: vec![],
        }
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
    Parent(Content),
}

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug)]
enum ContentType {
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
struct Content {
    id: String,
    content: ContentType,
    links: Vec<Link>,
}

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug)]
enum Identifier {
    Person(Person),
    Place,
    Time(String),
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
    use crate::note::{Content, ContentType, Identifier, Link, Note, Person};

    #[test]
    fn test_note() {
        let n = Note {
            children: vec![],
            content: vec![
                Content {
                    id: "1234".to_string(),
                    content: ContentType::Identifier(Identifier::Time(String::from(
                        "January 19th, 2021",
                    ))),
                    links: vec![Link::Parent(Content {
                        id: "1234".to_string(),
                        content: ContentType::Note(Box::new(Note {
                            children: vec![],
                            content: vec![Content {
                                id: "1234".to_string(),
                                content: ContentType::Tag("ML".to_string()),
                                links: vec![],
                            }],
                        })),
                        links: vec![],
                    })],
                },
                // content: vec![ContentType::Tag(String::from("ML"))],
                // Content::Text(String::from("Call with guy at")),
                // Content::Tag(String::from("ISG")),
                // Content::Identifier(Identifier::Person(Person {
                //     name: "Peter Graham".to_string(),
                // })),
            ],
        };
        println!("{:#?}", n);
    }
}
