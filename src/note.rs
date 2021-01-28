/// A `Note` is meant to be a sentence or short paragraph. It serves
/// to associate series of `Content`s together.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug, Clone)]
pub struct Note {
    content: Vec<Content>,
}
impl Note {
    pub fn builder() -> NoteBuilder {
        NoteBuilder {
            content: Default::default(),
        }
    }
    pub fn get_content(&self) -> &Vec<Content> {
        &self.content
    }
}
pub struct NoteBuilder {
    content: Vec<Content>,
}
impl NoteBuilder {
    pub fn build(self) -> Note {
        Note {
            content: self.content.clone(),
        }
    }
    pub fn add_content(&mut self, content: ContentType) -> NoteBuilder {
        let mut new_content: Vec<Content> = self.content.clone();
        new_content.push(Content {
            id: "".to_string(),
            content,
            links: vec![],
        });
        NoteBuilder {
            content: new_content,
        }
    }
}
impl Default for Note {
    fn default() -> Self {
        Self { content: vec![] }
    }
}
impl ToString for Content {
    fn to_string(&self) -> String {
        self.content.to_string()
    }
}
impl ToString for Identifier {
    fn to_string(&self) -> String {
        match self {
            Identifier::Person(p) => p.to_string(),
            Identifier::Place(s) => String::from(s),
            Identifier::Time(t) => t.to_string(),
            Identifier::Business(s) => String::from(s),
            Identifier::Event => String::from("EVENT@TODO"),
        }
    }
}
impl ToString for ContentType {
    fn to_string(&self) -> String {
        match self {
            ContentType::Media(_) => String::from("MEDIA@TODO"),
            ContentType::Text(t) => String::from(t),
            ContentType::Tag(t) => String::from(t),
            ContentType::Identifier(i) => i.to_string(),
            ContentType::Link(_) => String::from("LINK@TODO"),
            ContentType::Image => String::from("Image@TODO"),
            ContentType::Note(_) => String::from("NOTE@TODO"),
            ContentType::Task(_) => String::from("TASK@TODO"),
        }
    }
}
impl ToString for Note {
    fn to_string(&self) -> String {
        let s: String = self
            .content
            .iter()
            .map(|content| content.to_string())
            .collect();
        s
    }
}
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug, Clone)]
pub struct Task {
    desc: String,
}
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug, Clone)]
pub enum Media {
    Image,
}

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug, Clone)]
pub enum Image {
    Attached,
    Linked,
}

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug, Clone)]
pub enum Link {
    Parent(Content),
}

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
pub struct Content {
    id: String,
    content: ContentType,
    links: Vec<Link>,
}

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug, Clone)]
pub enum Identifier {
    Person(Person),
    Event,
    Place(String),
    Time(String),
    Business(String),
}

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug, Clone)]
pub struct Person {
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
    fn test_builder() {
        let b = Note::builder()
            .add_content(ContentType::Text("Compose demo ready for ".to_string()))
            .add_content(ContentType::Identifier(Identifier::Business(
                "ISG".to_string(),
            )))
            .add_content(ContentType::Text(" meeting ".to_string()))
            .add_content(ContentType::Identifier(Identifier::Time(
                "January 28th, 2021".to_string(),
            )))
            .build();
        println!("Note: {:#?}", b);
    }
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
