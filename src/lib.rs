//! Simple Mermaid diagrams RustDoc integration
//!
//! This crate provides a simple declarative macro to include [mermaid] diagrams in your rustdoc documentation.
//! Lookup the great [mermaid documentation] for details on the diagram syntax.
//!
//! # Usage
//! 1. Create your [mermaid] diagrams in their own files (usually with `.mmd` or `.mermaid` extension).
//! 2. Call the [mermaid!] macro in a `#[doc]` attribute. Specify the route to the diagram file as a
//!    string literal. Note that the path is interpreted relative to the file where the macro is called,
//!    as it happens with the underlying [include_str].
//! 3. Done!
//!
//! Diagrams can be intermixed freely with regular documentation comments.
//!
//! ```rust
//! # use simple_mermaid::mermaid;
//! /// A sequence diagram
//! #[doc = mermaid!("sequence.mmd")]
//! ///
//! /// Then a flowchart
//! #[doc = mermaid!("flowchart.mmd")]
//! ///
//! /// And some more regular text to end this block
//! # fn function() {}
//! ```
//! Outputs:
//!
//! <pre>
//! A sequence diagram
#![doc = mermaid!("sequence.mmd")]
//! Then a flowchart
#![doc = mermaid!("flowchart.mmd")]
//! And some more regular text to end this block
//! </pre>
//!
//! # Options
//! By default, diagrams will be centered and have a transparent background. This behaviour can be
//! controlled with the following keywords after the path to the [mermaid] file:
//!
//! * **left**, left align the diagram.
//! * **right**, right align the diagram.
//! * **center**, has not effect, but it\"s accepted for completeness.
//! * **framed**, add a gray frame to the diagram.
//! * **transparent**, do not add the gray frame to the diagram.
//!
//! *Left*, *center* and *right* are, of course, mutually exclusive; but either can be combined with *framed*.
//!
//! ```rust
//! # use simple_mermaid::mermaid;
//! #[doc = mermaid!("er.mmd" left)]
//! #[doc = mermaid!("graph.mmd" framed)]
//! #[doc = mermaid!("timeline.mmd" right)]
//! #[doc = mermaid!("larger.mmd" center framed)]
//! # fn function() {}
//! ```
#![doc = mermaid!("er.mmd" left)]
#![doc = mermaid!("graph.mmd" framed)]
#![doc = mermaid!("timeline.mmd" right)]
#![doc = mermaid!("larger.mmd" center)]
//!
//! # Alternatives
//! ## aquamarine
//! The [aquamarine] introduces a procedural macro that converts regular code blocks marked with the
//! [mermaid] language tag. It also allows including the diagram from external files, but that comes with some limitations:
//! * Only one external diagram can be added to a single doc block.
//! * The external diagram will always appear at the end of the doc block.
//!
//! Those limitations made [aquamarine] a non-option for me, since I strongly prefer leaving the diagrams in external files for several reasons:
//! clutter, maintainability, IDE support, and, re-usability of the diagrams.
//!
//! Besides, the declarative macro used in this crate should be easier on compile times. And it comes with no dependencies at all!
//!
//! ## rsdoc
//! The [rsdoc] crate provides procedural macros to embed [PlantUML] and images in doc coments.
//! It can be used with code-blocks (similar to aquamarine) or with external files (similar to this crate).
//! So, in this case, for me it was just a matter of personal taste, both [PlantUML] and [mermaid] are fantastic
//! opensource projects. But PlantUML is Java... and my plants always die (_even a cactus I once had! How can a cactus die? The thing should not need water!_).
//!
//! # Disclaimer
//! Neither this crate nor it's autor have any relation or affiliation with the [mermaid] project, other that being an user of this magnific library.
//!
//! All the examples in this documentation have been extracted, verbatim or with minor updates, from the [mermaid documentation].
//!
//! [mermaid]: https://mermaid.js.org/
//! [include_str]: https://doc.rust-lang.org/std/macro.include_str.html
//! [mermaid documentation]: https://mermaid.js.org/intro/n00b-gettingStarted.html
//! [aquamarine]: https://crates.io/crates/aquamarine
//! [rsdoc]: https://crates.io/crates/rsdoc
//! [PlantUML]: https://plantuml.com/

#![no_std]

/// Include a mermaid diagram in the documentation.
///
/// This macro is meant to be used as argument of the `#[doc]` attribute.
///
/// ```rust
/// # use simple_mermaid::mermaid;
/// /// Some documentation about a function
/// /// Then include a diagram:
/// #[doc = mermaid!("netflix.mmd")]
/// fn a_function() {}
/// ```
///
/// This would produce:
///
/// > Some documentation
/// >
/// > Then include a diagram:
#[doc = mermaid!("netflix.mmd" framed)]
///
/// Look at the crate level documentation for all the options.
#[macro_export]
macro_rules! mermaid {
    ($file:literal)               => { $crate::_mermaid_inner!($file center transparent) };
    ($file:literal left framed)   => { $crate::_mermaid_inner!($file left framed) };
    ($file:literal framed left)   => { $crate::_mermaid_inner!($file left framed) };
    ($file:literal center framed) => { $crate::_mermaid_inner!($file center framed) };
    ($file:literal framed center) => { $crate::_mermaid_inner!($file center framed) };
    ($file:literal right framed)  => { $crate::_mermaid_inner!($file right framed) };
    ($file:literal framed right)  => { $crate::_mermaid_inner!($file right framed) };
    ($file:literal framed)        => { $crate::_mermaid_inner!($file center framed) };
    ($file:literal left)          => { $crate::_mermaid_inner!($file left transparent) };
    ($file:literal right)         => { $crate::_mermaid_inner!($file right transparent) };
    ($file:literal center)        => { $crate::_mermaid_inner!($file center transparent) };
}

#[doc(hidden)]
#[macro_export]
macro_rules! _mermaid_inner {
    ($file:literal $pos:ident $style:ident)  =>
    {
        concat!("<pre class=\"mermaid\" style=\"text-align:", stringify!($pos), ";", $crate::_mermaid_background!($style), "\">\n",
                    include_str!($file), "\n",
                "</pre>",
                "<script type=\"module\">",
                    "import mermaid from \"https://cdn.jsdelivr.net/npm/mermaid@10/dist/mermaid.esm.min.mjs\";",
                    "var doc_theme = localStorage.getItem(\"rustdoc-theme\");",
                    "if (doc_theme === \"dark\" || doc_theme === \"ayu\") mermaid.initialize({theme: \"dark\"});",
                "</script>")
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! _mermaid_background {
    (framed) =>  { "" };
    (transparent) => { "background: transparent;" };
}
