#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_should_build_a_valid_html_tag () {
        
        let tag = Component::new("div", vec![
            Attribute::KV(("id", "my-tag-id")),
            Attribute::KV(("class", "my-class my-second-class")),
            Attribute::KV(("onclick", "function () {{ console.log('click, click') }}"))
        ]);

        let expected = "<div id='my-tag-id' class='my-class my-second-class' onclick='function () {{ console.log('click, click') }}'></div>";
        assert_eq!(tag.build(), expected);

    }

    #[test]
    fn it_should_build_a_valid_html_tag_with_child () {
        
        let mut tag = Component::new("div", vec![
            Attribute::KV(("id", "my-tag-id"))
        ]);
        let tag2 = Component::new("p", vec![
            Attribute::KV(("id", "child-id"))
        ]);
        tag = tag.add_child(tag2);

        let expected = "<div id='my-tag-id'><p id='child-id'></p></div>";
        assert_eq!(tag.build(), expected);

    }

    #[test]
    fn it_should_build_a_valid_html_tag_with_child_and_text () {
        
        let mut tag = Component::new("div", vec![
            Attribute::KV(("id", "my-tag-id"))
        ]);
        let mut tag2 = Component::new("p", vec![
            Attribute::KV(("id", "child-id"))
        ]);
        tag = tag.add_text("text");
        tag2 = tag2.add_text("hello, from inside the p tag");
        tag = tag.add_child(tag2);
        tag = tag.add_text("text again");
        let expected = "<div id='my-tag-id'>text<p id='child-id'>hello, from inside the p tag</p>text again</div>";
        assert_eq!(tag.build(), expected);

    }

    #[test]
    fn it_should_build_a_video_tag () {
        
        let mut video_tag = Component::new("video", vec![
            Attribute::V("controls"),
            Attribute::KV(("width", "250")),
        ]);
        let source_tag = Component::new("source", vec![
            Attribute::KV(("src", "/media/examples/flower.webm")),
            Attribute::KV(("type", "video/webm")),
        ]);
        video_tag = video_tag.add_child(source_tag);
        
        let expected = "<video controls width='250'><source src='/media/examples/flower.webm' type='video/webm'></video>";
        assert_eq!(video_tag.build(), expected);

    }

    #[test]
    fn it_should_build_a_form () {

        let form_builder = Component::new("form", vec![ Attribute::KV(("action", "")), Attribute::KV(("method", "get")), Attribute::KV(("class", "form-example")) ])

            // add div to form
            .add_child(Component::new("div", vec![ Attribute::KV(("class", "form-example")) ])
                // add lable for input
                .add_child(Component::new("label", vec![Attribute::KV(("for", "name"))]).add_text("Enter your name: "))
                // add input for name
                .add_child(Component::new("input", vec![Attribute::KV(("type", "text")), Attribute::KV(("name", "name")), Attribute::KV(("id", "name")), Attribute::V("required")])))
                // add div to form
            
            // add div for email
            .add_child(Component::new("div", vec![ Attribute::KV(("class", "form-example")) ])
                // add lable for input
                .add_child(Component::new("label", vec![Attribute::KV(("for", "email"))]).add_text("Enter your email: "))
                // add input for name
                .add_child(Component::new("input", vec![Attribute::KV(("type", "email")), Attribute::KV(("name", "email")), Attribute::KV(("id", "email")), Attribute::V("required")])))
            
            // add div for submit
            .add_child(Component::new("div", vec![ Attribute::KV(("class", "form-example")) ])
                .add_child(Component::new("input", vec![ Attribute::KV(("type", "submit")), Attribute::KV(("value", "Subscribe!")) ])));

        let form = form_builder.build();
        let expected = "<form action='' method='get' class='form-example'><div class='form-example'><label for='name'>Enter your name: </label><input type='text' name='name' id='name' required></div><div class='form-example'><label for='email'>Enter your email: </label><input type='email' name='email' id='email' required></div><div class='form-example'><input type='submit' value='Subscribe!'></div></form>";
        assert_eq!(form, expected);

    }

}

pub enum InnerHTML <T1, T2> {
    Component(T1),
    Text(T2)
}

pub enum Attribute {
    KV((&'static str, &'static str)),
    V(&'static str)
}

type Attributes = Vec<Attribute>;
type NodeInnerHTML = Vec<InnerHTML<Component, &'static str>>;

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

    pub fn add_child (mut self, child: Component) -> Component {
        self.inner_html.push(InnerHTML::Component(child));
        self
    }

    pub fn add_text (mut self, text: &'static str) -> Component {
        self.inner_html.push(InnerHTML::Text(text));
        self
    }


}
