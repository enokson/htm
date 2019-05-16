#[cfg(test)]
pub mod tests {

    use super::super::component::{ Attribute, Component };
    use super::super::cssbox::{ CssBox, Property };
    use super::super::stylesheet::{ Entry, StyleSheet };

    #[test]
    fn it_should_build_a_valid_html_tag () {
        
        let tag_builder = Component::new("div", vec![
            Attribute::KV(("id", "my-tag-id")),
            Attribute::KV(("class", "my-class my-second-class")),
            Attribute::KV(("onclick", "function () { console.log('click, click') }")) ]);

        let expected = "<div id='my-tag-id' class='my-class my-second-class' onclick='function () { console.log('click, click') }'></div>";
        assert_eq!(tag_builder.build(), expected);

    }

    #[test]
    fn it_should_build_a_valid_html_tag_with_child () {
        
        let tag_builder = Component::new("div", vec![ Attribute::KV(("id", "my-tag-id")) ])
            .child(Component::new("p", vec![ 
                Attribute::KV(("id", "child-id")) ]));

        let expected = "<div id='my-tag-id'><p id='child-id'></p></div>";
        assert_eq!(tag_builder.build(), expected);

    }

    #[test]
    fn it_should_build_a_valid_html_tag_with_child_and_text () {
        
        let tag_builder = Component::new("div", vec![ Attribute::KV(("id", "my-tag-id")) ])
            .text("text")
            .child(Component::new("p", vec![ 
                Attribute::KV(("id", "child-id")) ])
                    .text("hello, from inside the p tag")
            )
            .text("more text");
        let expected = "<div id='my-tag-id'>text<p id='child-id'>hello, from inside the p tag</p>more text</div>";
        assert_eq!(tag_builder.build(), expected);

    }

    #[test]
    fn it_should_build_a_video_tag () {
        
        let video_tag_builder = Component::new("video", vec![
            Attribute::V("controls"),
            Attribute::KV(("width", "250")),
        ])
            .child(Component::new("source", vec![
                Attribute::KV(("src", "/media/examples/flower.webm")),
                Attribute::KV(("type", "video/webm")),
            ]));
        
        let expected = "<video controls width='250'><source src='/media/examples/flower.webm' type='video/webm'></video>";
        assert_eq!(video_tag_builder.build(), expected);

    }

    #[test]
    fn it_should_build_a_form () {

        let form_builder = Component::new("form", vec![ 
            Attribute::KV(("action", "")), 
            Attribute::KV(("method", "get")), 
            Attribute::KV(("class", "form-example")) ])

            // add div to form for name
            .child(
                Component::new("div", vec![ Attribute::KV(("class", "form-example")) ])
                    // add lable for input
                    .child(
                        Component::new("label", vec![Attribute::KV(("for", "name"))])
                            .text("Enter your name: ")
                    )
                    // add input for name
                    .child(
                        Component::new("input", vec![
                            Attribute::KV(("type", "text")), 
                            Attribute::KV(("name", "name")), 
                            Attribute::KV(("id", "name")), 
                            Attribute::V("required")])
                    )
            )
            
            // add div for email
            .child(
                Component::new("div", vec![ Attribute::KV(("class", "form-example")) ])
                    // add lable for input
                    .child(
                        Component::new("label", vec![Attribute::KV(("for", "email"))]).text("Enter your email: ")
                    )
                    // add input for name
                    .child(
                        Component::new("input", vec![
                            Attribute::KV(("type", "email")), 
                            Attribute::KV(("name", "email")), 
                            Attribute::KV(("id", "email")), 
                            Attribute::V("required")])
                    )
            )
            
            // add div for submit
            .child(Component::new("div", vec![ Attribute::KV(("class", "form-example")) ])
                .child(Component::new("input", vec![ 
                    Attribute::KV(("type", "submit")), 
                    Attribute::KV(("value", "Subscribe!")) ])
                )
            );

        let form = form_builder.build();
        let expected = "<form action='' method='get' class='form-example'><div class='form-example'><label for='name'>Enter your name: </label><input type='text' name='name' id='name' required></div><div class='form-example'><label for='email'>Enter your email: </label><input type='email' name='email' id='email' required></div><div class='form-example'><input type='submit' value='Subscribe!'></div></form>";
        assert_eq!(form, expected);

    }

    #[test]
    fn it_should_build_a_class () {
        let class = CssBox::new(".p", vec![
            Property::Declaration(("height", "250 px"))
        ]);
        assert_eq!(class.build(), ".p {height: 250 px;}");
    }

    #[test]
    fn it_should_build_a_script_tag () {
        let style_builder = StyleSheet::new(vec![
            Entry::CssBox(CssBox::new(".my-class", vec![ Property::Declaration(("height", "250 vh")), Property::Declaration(("width", "100 px"))]))
        ]).css_box(CssBox::new(".my-second-class", vec![])
            .declaration("color", "red"));
        assert_eq!(style_builder.build(), "<style>.my-class {height: 250 vh;width: 100 px;}.my-second-class {color: red;}</style>");
    }

}