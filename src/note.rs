/// A `Note` is meant to be a sentence or short paragraph. It serves
/// to associate series of `Content`s together.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug)]
pub struct Note {
    content: Vec<Content>,
}

impl Default for Note {
    fn default() -> Self {
        Self { content: vec![] }
    }
}

impl ToString for Note {
    fn to_string(&self) -> String {
        let s: String = self
            .content
            .iter()
            .map(|content| match &content.content {
                ContentType::Media(_) => String::from("MEDIA@TODO"),
                ContentType::Text(t) => String::from("TEXT@TODO"),
                ContentType::Tag(t) => String::from("TAG@TODO"),
                ContentType::Identifier(i) => match i {
                    Identifier::Person(p) => p.to_string(),
                    Identifier::Place => String::from("PLACE@TODO"),
                    Identifier::Time(t) => t.to_string(),
                    Identifier::Thing => String::from("THING@TODO"),
                    Identifier::Event => String::from("EVENT@TODO"),
                },
                ContentType::Link(_) => String::from("LINK@TODO"),
                ContentType::Image => String::from("Image@TODO"),
                ContentType::Note(_) => String::from("NOTE@TODO"),
                ContentType::Task(_) => String::from("TASK@TODO"),
            })
            .collect();
        s
    }
}
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug)]
pub struct Task {
    desc: String,
}
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug)]
pub enum Media {
    Image,
}

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug)]
pub enum Image {
    Attached,
    Linked,
}

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug)]
pub enum Link {
    Parent(Content),
}

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug)]
pub enum ContentType {
    Media(Media),
    Text(String),
    Tag(String),
    Identifier(Identifier),
    Link(String),
    Image,
    Note(Box<Note>),
    Task(Task),
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
    Event,
    Place,
    Time(String),
    Thing,
}

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug)]
struct Person {
    name: String,
}
impl ToString for Person {
    fn to_string(&self) -> String {
        self.name.to_string()
    }
}

// Only `Note`s can have children, but all content can have links?
//
#[cfg(test)]
mod test {
    use crate::note::{Content, ContentType, Identifier, Link, Note, Person};

    #[test]
    fn test_note() {
        let n = Note {
            content: vec![
                Content {
                    id: "1234".to_string(),
                    content: ContentType::Identifier(Identifier::Time(String::from(
                        "January 19th, 2021",
                    ))),
                    links: vec![Link::Parent(Content {
                        id: "1234".to_string(),
                        content: ContentType::Note(Box::new(Note {
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
        println!("Note: {:#?}", n.to_string());
    }
}
