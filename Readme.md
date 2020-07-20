# An exploration of rust GUIs 🔥

## 👉 Iced

First in the list is [Iced](https://github.com/hecrj/iced) -

> "A cross-platform GUI library for Rust focused on simplicity and type-safety. Inspired by Elm."

Iced provides an api which is very similar to elm architecture. Of all the gui toolkits
I have came accross in Rust, `iced` felt the most polished and versatile among all. It has the most community support as well.

![Alt text](/screenshots/Iced_gui.png?raw=true "Iced Screenshot 1")

Iced provides **good documentation** and a rich selection of **examples** and feature showcases. While building the app I found everything I needed in the examples. Rarely needed to jump into api docs. Which is very good for begineers.

The logic and the render loop of the ui is akin to elm architecture.
You define your `application state` -> `render views` -> `views` can react to ( send
messages ) `events` -> these messages will in turn update your application state
and ultimately render your view again according to the new state.

Iced has great support for `styling` components as well.
Layout is still basic with only support for box model and horizontal/vertical
containers. Font and other resource handling was smooth. Although there's still no support for
multiline texts as multiline text feature requires text reshaping and other very
hard to implement features [Text rendering hates you](https://gankra.github.io/blah/text-hates-you/).
Overall if you don't need to build an editor or something similar, Iced should
work out for you pretty well.

Helpful Links

- [Examples](https://github.com/hecrj/iced/tree/master/examples)
- [Iced Project Showcase](https://github.com/hecrj/iced/issues/355)
