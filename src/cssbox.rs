
pub type Declaration = (String, String);
pub type Declarations = Vec<Declaration>;

pub enum Property {
    CssBox(CssBox),
    Declaration(Declaration)
}

type Properties = Vec<Property>;

pub struct CssBox {
    pub selector: String,
    pub properties: Properties
}

impl CssBox {

    pub fn new (selector: &str) -> CssBox {
        CssBox {
            selector: selector.to_string(),
            properties: vec![]
        }
    }

    pub fn declaration (mut self, key: &str, value: &str) -> CssBox {
        self.properties.push(Property::Declaration((key.to_string(), value.to_string())));
        self
    }

    pub fn css_box (mut self, css_box: CssBox) -> CssBox {
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
