use super::cssbox::{ CssBox };
use super::component::{ Slice };

pub type CssBoxes = Vec<CssBox>;
pub enum Entry {
    CssBox(CssBox),
    Rule(String)
}
pub type Entries = Vec<Entry>;


/// A style tag for embedded css
/// 
/// ```
/// use htm::stylesheet::{ Entry, StyleSheet };
/// use htm::cssbox::{ CssBox, Property };
/// 
/// let class_name = ".my-class".to_string();
/// let height = ("height".to_string(), "250vh".to_string());
/// 
/// let stylesheet_builder = StyleSheet::new(vec![
///     Entry::CssBox(CssBox::new(".my-class".to_string(), vec![
///         Property::Declaration(height),
///         Property::Declaration(("width".to_string(), "100px".to_string()))
///     ])),
///     Entry::CssBox(CssBox::new(".my-second-class".to_string(), vec![
///         Property::Declaration(("color".to_string(), "red".to_string()))
///     ]))
/// ]);
/// 
/// let stylesheet = stylesheet_builder.build();
/// 
/// assert_eq!(stylesheet, "<style>.my-class {height: 250vh;width: 100px;}.my-second-class {color: red;}</style>");
/// 
/// //
/// // <style>
/// //      .my-class {
/// //          height: 250vh;
/// //          width: 100px;
/// //      }
/// //
/// //      .my-second-class {
/// //          color: red;
/// //      }
/// //</style>
/// //
/// ```
/// 
/// Using @rules and method chainging style
/// ```
/// use htm::stylesheet::{ Entry, StyleSheet };
/// use htm::cssbox::{ CssBox, Property };
/// 
/// let stylesheet_builder = StyleSheet::new(vec![])
///     .at_rule("@import 'custom.css'".to_string()) // the simicolon will be added automatically when build is ran
///     .css_box(CssBox::new(".my-class".to_string(), vec![
///         Property::Declaration(("height".to_string(), "250vh".to_string())),
///         Property::Declaration(("width".to_string(), "100px".to_string()))]))
///     .css_box(CssBox::new(".my-second-class".to_string(), vec![
///         Property::Declaration(("color".to_string(), "red".to_string())) ]));
/// 
/// 
/// let stylesheet = stylesheet_builder.build();
/// 
/// assert_eq!(stylesheet, "<style>@import 'custom.css';.my-class {height: 250vh;width: 100px;}.my-second-class {color: red;}</style>");
/// 
/// //
/// // <style>
/// //      @import 'custom.css';
/// //      .my-class {
/// //          height: 250vh;
/// //          width: 100px;
/// //      }
/// //
/// //      .my-second-class {
/// //          color: red;
/// //      }
/// //</style>
/// //
/// ```
/// 
/// Nesting css boxes 
/// ```
/// use htm::stylesheet::{ Entry, StyleSheet };
/// use htm::cssbox::{ CssBox, Property };
/// 
/// let stylesheet_builder = StyleSheet::new(vec![])
///     .css_box(CssBox::new("@media (min-width: 801px)".to_string(), vec![])
///         .css_box(CssBox::new("body".to_string(), vec![])
///             .declaration("color".to_string(), "red".to_string())));
/// 
/// 
/// let stylesheet = stylesheet_builder.build();
/// 
/// assert_eq!(stylesheet, "<style>@media (min-width: 801px) {body {color: red;}}</style>");
/// 
/// //
/// // <style>
/// //     @media (min-width: 801px) {
/// //          body {
/// //              color: red;
/// //          }
/// //     }
/// //</style>
/// //
/// ```
/// Using multiple selectors
/// ```
/// use htm::stylesheet::{ Entry, StyleSheet };
/// use htm::cssbox::{ CssBox, Property };
/// 
/// let stylesheet_builder = StyleSheet::new(vec![])
///     .css_box(CssBox::new("#my-id, .my-class, p.my-second-class".to_string(), vec![])
///         .declaration("color".to_string(), "red".to_string()));
/// 
/// let stylesheet = stylesheet_builder.build();
/// 
/// assert_eq!(stylesheet, "<style>#my-id, .my-class, p.my-second-class {color: red;}</style>");
/// 
/// //
/// // <style>
/// //      #my-id, .my-class, p.my-second-clas {
/// //          color: red;
/// //      }
/// //</style>
/// //
/// ```
/// 
pub struct StyleSheet {
    lines: Entries
}

impl StyleSheet {

    pub fn new (lines: Entries) -> StyleSheet {
        StyleSheet {
            lines
        }
    }

    pub fn css_box (mut self, css_box: CssBox) -> StyleSheet {
        self.lines.push(Entry::CssBox(css_box));
        self
    }

    pub fn at_rule (mut self, rule: String) -> StyleSheet {
        self.lines.push(Entry::Rule(rule));
        self
    }

    pub fn build (&self) -> String {
        let mut stylesheet = format!("<style>");
        for ln in self.lines.iter() {
            match ln {
                Entry::CssBox(css_box) => {
                    stylesheet = format!("{}{}", stylesheet, css_box.build());
                },
                Entry::Rule(rule) => {
                    stylesheet = format!("{}{};", stylesheet, rule);
                }
            }
            
        }
        stylesheet = format!("{}</style>", stylesheet);
        stylesheet
    }
}