use super::cssbox::{ CssBox };
use super::component::{ Slice };

pub type CssBoxes<'a> = Vec<CssBox<'a>>;
pub enum Entry<'a> {
    CssBox(CssBox<'a>),
    Rule(Slice<'a>)
}
pub type Entries<'a> = Vec<Entry<'a>>;


/// A style tag for embedded css
/// 
/// ```
/// use htm::stylesheet::{ Entry, StyleSheet };
/// use htm::cssbox::{ CssBox, Property };
/// 
/// let stylesheet_builder = StyleSheet::new(vec![
///     Entry::CssBox(CssBox::new(".my-class", vec![
///         Property::Declaration(("height", "250vh")),
///         Property::Declaration(("width", "100px"))
///     ])),
///     Entry::CssBox(CssBox::new(".my-second-class", vec![
///         Property::Declaration(("color", "red"))
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
///     .at_rule("@import 'custom.css'") // the simicolon will be added automatically when build is ran
///     .css_box(CssBox::new(".my-class", vec![
///         Property::Declaration(("height", "250vh")),
///         Property::Declaration(("width", "100px"))]))
///     .css_box(CssBox::new(".my-second-class", vec![
///         Property::Declaration(("color", "red")) ]));
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
///     .css_box(CssBox::new("@media (min-width: 801px)", vec![])
///         .css_box(CssBox::new("body", vec![])
///             .declaration("color", "red")));
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
///     .css_box(CssBox::new("#my-id, .my-class, p.my-second-class", vec![])
///         .declaration("color", "red"));
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
pub struct StyleSheet<'a> {
    lines: Entries<'a>
}

impl<'a> StyleSheet<'a> {

    pub fn new (lines: Entries<'a>) -> StyleSheet<'a> {
        StyleSheet {
            lines
        }
    }

    pub fn css_box (mut self, css_box: CssBox<'a>) -> StyleSheet<'a> {
        self.lines.push(Entry::CssBox(css_box));
        self
    }

    pub fn at_rule (mut self, rule: Slice<'a>) -> StyleSheet<'a> {
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