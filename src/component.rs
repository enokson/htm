
pub enum InnerHTML {
    Component(Component),
    Text(String)
}

pub enum Attribute {
    KV(KV),
    V(V)
}

pub type Slice<'a> = &'a str;
pub type KV = (String, String);
pub type V = String;
pub type Attributes = Vec<Attribute>;
pub type NodeInnerHTML = Vec<InnerHTML>;

/// An html component
///
/// # Examples
/// Creating a simple p tag with two attributes (id and class) with some text in the inner html
/// ```rust
/// use htm::component::{ Attribute, Component};
/// 
/// let p_builder = Component::new("p".to_string(), vec![ 
///     Attribute::KV(("id".to_string(), "my-id".to_string())), 
///     Attribute::KV(("class".to_string(), "my-class".to_string()))])
///     .text("Hello, world!".to_string());
/// let p = p_builder.build();
///
/// assert_eq!(p, "<p id='my-id' class='my-class'>Hello, world!</p>");
/// // <p id='my-id' class='my-class'>Hello, world!</p>
/// ```
///
///
/// Create a form tag
/// ```
/// use htm::component::{ Attribute, Component};
///
/// let form_builder = Component::new("form".to_string(), vec![
///     Attribute::KV(("action".to_string(), "".to_string()))])
///     .child(Component::new("label".to_string(), vec![ 
///         Attribute::KV(("for".to_string(), "test-input".to_string())) ])
///         .text("Enter name: ".to_string()))
///     .child(Component::new("input".to_string(), vec![
///         Attribute::KV(("type".to_string(), "text".to_string())),
///         Attribute::KV(("name".to_string(), "test-input".to_string())),
///         Attribute::KV(("id".to_string(), "test-input".to_string())),
///         Attribute::V("required".to_string())]));
/// 
/// let form = form_builder.build();
/// assert_eq!(form, "<form action=''><label for='test-input'>Enter name: </label><input type='text' name='test-input' id='test-input' required></form>");
///
/// // result:
/// // <form action=''>
/// //      <label for='test-input'>Enter name: </label>
/// //      <input type='text' name='test-input' id='test-input' required>
/// //  </form>
/// ```
/// 
/// Create a video tag
/// ```
/// use htm::component::{ Attribute, Component };
/// 
/// let video_builder = Component::new("video".to_string(), vec![
///     Attribute::V("controls".to_string()),
///     Attribute::KV(("width".to_string(), "250".to_string()))])
///     .child(Component::new("source".to_string(), vec![
///         Attribute::KV(("src".to_string(), "/media/examples/flower.webm".to_string())),
///         Attribute::KV(("type".to_string(), "video/webm".to_string()))
///     ])) 
///     .child(Component::new("source".to_string(), vec![
///         Attribute::KV(("src".to_string(), "/media/examples/flower.mp4".to_string())),
///         Attribute::KV(("type".to_string(), "video/mp4".to_string()))
///     ])) 
///     .text("Sorry, your browser doesn't support embedded videos.".to_string());
/// 
/// assert_eq!(video_builder.build(), "<video controls width='250'><source src='/media/examples/flower.webm' type='video/webm'><source src='/media/examples/flower.mp4' type='video/mp4'>Sorry, your browser doesn't support embedded videos.</video>");
/// 
/// // result: 
/// // <video controls width="250">
/// // 
/// //     <source src="/media/examples/flower.webm" type="video/webm">
/// // 
/// //     <source src="/media/examples/flower.mp4" type="video/mp4">
/// // 
/// //     Sorry, your browser doesn't support embedded videos.
/// //
/// // </video>
/// 
/// ```
/// 
/// Using method chaining
/// ```
/// use htm::component::{ Attribute, Component };
///
/// let form_builder = Component::new("form".to_string(), vec![])
///     .kv_attr("action".to_string(), "".to_string())
///     .child(Component::new("label".to_string(), vec![])
///         .kv_attr("for".to_string(), "test-input".to_string())
///         .text("Enter name: ".to_string()))
///     .child(Component::new("input".to_string(), vec![])
///         .kv_attr("type".to_string(), "text".to_string())
///         .kv_attr("name".to_string(), "test-input".to_string())
///         .kv_attr("id".to_string(), "test-input".to_string())
///         .v_attr("required".to_string()));
/// 
/// let form = form_builder.build();
/// assert_eq!(form, "<form action=''><label for='test-input'>Enter name: </label><input type='text' name='test-input' id='test-input' required></form>");
///
/// // <form action=''>
/// //      <label for='test-input'>Enter name: </label>
/// //      <input type='text' name='test-input' id='test-input' required>
/// //  </form>
/// ```
pub struct Component {
    pub node_type: String,
    pub attributes: Attributes,
    pub inner_html: NodeInnerHTML
}

impl Component {

    pub fn new (node_type: String, attributes: Attributes) -> Component {
        Component {
            node_type,
            attributes,
            inner_html: vec![]
        }
    }

    pub fn build (&self) -> String {

        let mut node = format!("<{}", self.node_type);
        for entry in self.attributes.iter() {

            match entry {

                Attribute::KV((k, v)) => {
                    node = format!("{} {}='{}'", node, k, v);
                },
                Attribute::V(attr) => {
                    node = format!("{} {}", node, attr);
                }

            }
        }
        node = format!("{}>", node);
        for action in self.inner_html.iter() {

            match action {
                InnerHTML::Component(child) => {
                    node = format!("{}{}", node, child.build());
                },
                InnerHTML::Text(text) => {
                    node = format!("{}{}", node, text);
                }
            }

        }

        // close non-void tags
        if  self.node_type != "br" && 
            self.node_type != "hr" && 
            self.node_type != "img" && 
            self.node_type != "input" && 
            self.node_type != "link" && 
            self.node_type != "meta" && 
            self.node_type != "area" && 
            self.node_type != "base" && 
            self.node_type != "col" && 
            self.node_type != "command" && 
            self.node_type != "embed" && 
            self.node_type != "keygen" && 
            self.node_type != "param" && 
            self.node_type != "source" && 
            self.node_type != "track" && 
            self.node_type != "wbr" {

            node = format!("{}</{}>", node, self.node_type); 

        }
        node
    }

    pub fn kv_attr (mut self, key: String, value: String) -> Component {
        self.attributes.push(Attribute::KV((key, value)));
        self
    }

    pub fn v_attr (mut self, value: String) -> Component {
        self.attributes.push(Attribute::V(value));
        self
    }

    pub fn child (mut self, child: Component) -> Component {
        self.inner_html.push(InnerHTML::Component(child));
        self
    }

    pub fn text (mut self, text: String) -> Component {
        self.inner_html.push(InnerHTML::Text(text));
        self
    }


}