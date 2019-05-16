# HTM

# Examples
## Components
Creating a simple p tag with two attributes (id and class) with some text in the inner html
```rust
use htm::component::{ Attribute, Component};

let p_builder = Component::new("p", vec![ 
    Attribute::KV(("id", "my-id")), 
    Attribute::KV(("class", "my-class"))])
    .text("Hello, world!");
let p = p_builder.build();
```
```html
<!-- result -->
<p id='my-id' class='my-class'>Hello, world!</p>
```
Create a form tag
```rust
use htm::component::{ Attribute, Component};
let form_builder = Component::new("form", vec![
    Attribute::KV(("action", ""))])
    .child(Component::new("label", vec![ 
        Attribute::KV(("for", "test-input")) ])
        .text("Enter name: "))
    .child(Component::new("input", vec![
        Attribute::KV(("type", "text")),
        Attribute::KV(("name", "test-input")),
        Attribute::KV(("id", "test-input")),
        Attribute::V("required")]));

let form = form_builder.build();
```
```html
<!-- result -->
<form action=''>
    <label for='test-input'>Enter name: </label>
    <input type='text' name='test-input' id='test-input' required>
</form>
```

Create a video tag
```rust
use htm::component::{ Attribute, Component};

let video_builder = Component::new("video", vec![
    Attribute::V("controls"),
    Attribute::KV(("width", "250"))])
    .child(Component::new("source", vec![
        Attribute::KV(("src", "/media/examples/flower.webm")),
        Attribute::KV(("type", "video/webm"))
    ]))
    .child(Component::new("source", vec![
        Attribute::KV(("src", "/media/examples/flower.mp4")),
        Attribute::KV(("type", "video/mp4"))
    ]))
    .text("Sorry, your browser doesn't support embedded videos.");
let video_tag = video_builder.build();
```
```html
<!-- result -->
<video controls width="250">

    <source src="/media/examples/flower.webm" type="video/webm">

    <source src="/media/examples/flower.mp4" type="video/mp4">

    Sorry, your browser doesn't support embedded videos.

</video>
```
Using method chaining
```rust
use htm::component::{ Attribute, Component };
let form_builder = Component::new("form", vec![])
    .kv_attr("action", "")
    .child(Component::new("label", vec![])
        .kv_attr("for", "test-input")
        .text("Enter name: "))
    .child(Component::new("input", vec![])
        .kv_attr("type", "text")
        .kv_attr("name", "test-input")
        .kv_attr("id", "test-input")
        .v_attr("required"));

let form = form_builder.build();
```
```html
<!-- result -->
<form action=''>
    <label for='test-input'>Enter name: </label>
    <input type='text' name='test-input' id='test-input' required>
</form>
```

## CSS Stylesheets
A style tag for embedded css
```rust
use htm::stylesheet::{ Entry, StyleSheet };
use htm::cssbox::{ CssBox, Property };

let stylesheet_builder = StyleSheet::new(vec![
    Entry::CssBox(CssBox::new(".my-class", vec![
        Property::Declaration(("height", "250vh")),
        Property::Declaration(("width", "100px"))
    ])),
    Entry::CssBox(CssBox::new(".my-second-class", vec![
        Property::Declaration(("color", "red"))
    ]))
]);

let stylesheet = stylesheet_builder.build();
```
```html
<!-- result -->
<style>
      .my-class {
          height: 250vh;
          width: 100px;
      }

      .my-second-class {
          color: red;
      }
</style>
```

Using @rules and method chainging style
```rust
use htm::stylesheet::{ Entry, StyleSheet };
use htm::cssbox::{ CssBox, Property };

let stylesheet_builder = StyleSheet::new(vec![])
    .at_rule("@import 'custom.css'") // the simicolon will be added automatically when build is ran
    .css_box(CssBox::new(".my-class", vec![
        Property::Declaration(("height", "250vh")),
        Property::Declaration(("width", "100px"))]))
    .css_box(CssBox::new(".my-second-class", vec![
        Property::Declaration(("color", "red")) ]));

let stylesheet = stylesheet_builder.build();
```
```html
<!-- result -->
<style>
     @import 'custom.css';
     .my-class {
         height: 250vh;
         width: 100px;
     }
     .my-second-class {
         color: red;
     }
</style>
```

Nesting css boxes 
```rust
use htm::stylesheet::{ Entry, StyleSheet };
use htm::cssbox::{ CssBox, Property };

let stylesheet_builder = StyleSheet::new(vec![])
    .css_box(CssBox::new("@media (min-width: 801px)", vec![])
        .css_box(CssBox::new("body", vec![])
            .declaration("color", "red")));

let stylesheet = stylesheet_builder.build();
```

```html
<!-- result -->
<style>
    @media (min-width: 801px) {
        body {
            color: red;
        }
    }
</style>
```

Using multiple selctors
```rust
use htm::stylesheet::{ Entry, StyleSheet };
use htm::cssbox::{ CssBox, Property };

let stylesheet_builder = StyleSheet::new(vec![])
    .css_box(CssBox::new("#my-id, .my-class, p.my-second-class", vec![])
        .declaration("color", "red"));

let stylesheet = stylesheet_builder.build();
```
```html
<style>
    #my-id, .my-class, p.my-second-clas {
        color: red;
    }
</style>
```

