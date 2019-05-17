use super::component::{ Slice };

pub type Declaration<'a> = (Slice<'a>, Slice<'a>);
pub type Declarations<'a> = Vec<Declaration<'a>>;

pub enum Property<'a> {
    CssBox(CssBox<'a>),
    Declaration(Declaration<'a>)
}

type Properties<'a> = Vec<Property<'a>>;

pub struct CssBox<'a> {
    pub selector: Slice<'a>,
    pub properties: Properties<'a>
}

impl<'a> CssBox<'a> {

    pub fn new (selector: Slice<'a>, properties: Properties<'a>) -> CssBox<'a> {
        CssBox {
            selector,
            properties
        }
    }

    pub fn declaration (mut self, key: Slice<'a>, value: Slice<'a>) -> CssBox<'a> {
        self.properties.push(Property::Declaration((key, value)));
        self
    }

    pub fn css_box (mut self, css_box: CssBox<'a>) -> CssBox<'a> {
        self.properties.push(Property::CssBox(css_box));
        self
    }

    pub fn build (&self) -> String {
        let mut inner_tag = format!("{} {{", self.selector);
        for property in self.properties.iter() {
            match property {
                Property::Declaration(declaratrion) => {
                    let (k, v) = declaratrion;
                    inner_tag = format!("{}{}: {};", inner_tag, k, v);
                },
                Property::CssBox(css_box) => {
                    inner_tag = format!("{}{}", inner_tag, css_box.build());
                }
            }
        }
        inner_tag = format!("{}}}", inner_tag);
        inner_tag
    } 

}
