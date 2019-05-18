use super::component::{ Component };
use super::stylesheet::{ StyleSheet };

pub enum HtmlEntry {
    Component(Component),
    StyleSheet(StyleSheet)
}
pub type Entries = Vec<HtmlEntry>;
pub struct Html {
    proceeds: Entries
}

impl Html {

    pub fn new () -> Html {
        Html {
            proceeds: vec![]
        }
    }

    pub fn component (mut self, component: Component) -> Html {
        self.proceeds.push(HtmlEntry::Component(component));
        self
    }

    pub fn stylesheet (mut self, stylesheet: StyleSheet) -> Html {
        self.proceeds.push(HtmlEntry::StyleSheet(stylesheet));
        self
    }

    pub fn build (&self) -> String {
        let mut header = String::from("<!DOCTYPE html>");
        self.proceeds.iter().map(|entry| {
            match entry {
                HtmlEntry::Component(component) => component.build(),
                HtmlEntry::StyleSheet(stylesheet) => stylesheet.build()
            }
        }).fold(header, |html, component_string| format!("{}{}", html, component_string))
    }

}