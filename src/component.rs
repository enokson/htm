
pub enum InnerHTML {
    Component(Component),
    Text(Slice)
}

pub enum Attribute {
    KV(KV),
    V(V)
}

pub type Slice = &'static str;
pub type KV = (Slice, Slice);
pub type V = Slice;
pub type Attributes = Vec<Attribute>;
pub type NodeInnerHTML = Vec<InnerHTML>;

/// An html component
///
/// # Examples
/// Creating a simple p tag with two attributes (id and class) with some text in the inner html
/// ```rust
/// use htm::component::{ Attribute, Component};
/// 
/// let p_builder = Component::new("p", vec![ 
///     Attribute::KV(("id", "my-id")), 
///     Attribute::KV(("class", "my-class"))])
///     .text("Hello, world!");
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
/// let form_builder = Component::new("form", vec![
///     Attribute::KV(("action", ""))])
///     .child(Component::new("label", vec![ 
///         Attribute::KV(("for", "test-input")) ])
///         .text("Enter name: "))
///     .child(Component::new("input", vec![
///         Attribute::KV(("type", "text")),
///         Attribute::KV(("name", "test-input")),
///         Attribute::KV(("id", "test-input")),
///         Attribute::V("required")]));
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
/// let video_builder = Component::new("video", vec![
///     Attribute::V("controls"),
///     Attribute::KV(("width", "250"))])
///     .child(Component::new("source", vec![
///         Attribute::KV(("src", "/media/examples/flower.webm")),
///         Attribute::KV(("type", "video/webm"))
///     ])) 
///     .child(Component::new("source", vec![
///         Attribute::KV(("src", "/media/examples/flower.mp4")),
///         Attribute::KV(("type", "video/mp4"))
///     ])) 
///     .text("Sorry, your browser doesn't support embedded videos.");
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
/// let form_builder = Component::new("form", vec![])
///     .kv_attr("action", "")
///     .child(Component::new("label", vec![])
///         .kv_attr("for", "test-input")
///         .text("Enter name: "))
///     .child(Component::new("input", vec![])
///         .kv_attr("type", "text")
///         .kv_attr("name", "test-input")
///         .kv_attr("id", "test-input")
///         .v_attr("required"));
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
    pub node_type: &'static str,
    pub attributes: Attributes,
    pub inner_html: NodeInnerHTML
}

impl Component {

    pub fn new (node_type: &'static str, attributes: Attributes) -> Component {
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

    pub fn kv_attr (mut self, key: Slice, value: Slice) -> Component {
        self.attributes.push(Attribute::KV((key, value)));
        self
    }

    pub fn v_attr (mut self, value: Slice) -> Component {
        self.attributes.push(Attribute::V(value));
        self
    }

    pub fn child (mut self, child: Component) -> Component {
        self.inner_html.push(InnerHTML::Component(child));
        self
    }

    pub fn text (mut self, text: &'static str) -> Component {
        self.inner_html.push(InnerHTML::Text(text));
        self
    }


}