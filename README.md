# simple-mermaid

Simple Mermaid diagrams RustDoc integration

This crate provides a simple declarative macro to include [mermaid] diagrams in your rustdoc documentation.
Lookup the great [mermaid documentation] for details on the diagram syntax.

## Usage
1. Create your [mermaid] diagrams in their own files (usually with `.mmd` or `.mermaid` extension).
2. Call the [mermaid!] macro in a `#[doc]` attribute. Specify the route to the diagram file as a
   string literal. Note that the path is interpreted relative to the file where the macro is called,
   as it happens with the underlying [include_str].
3. Done!

# Alternatives
## aquamarine
The [aquamarine] introduces a procedural macro that converts regular code blocks marked with the
[mermaid] language tag. It also allows including the diagram from external files, but that comes with some limitations:
* Only one external diagram can be added to a single doc block.
* The external diagram will always appear at the end of the doc block.

Those limitations made [aquamarine] a non-option for me, since I strongly prefer leaving the diagrams in external files for several reasons:
clutter, maintainability, IDE support, and, re-usability of the diagrams.

Besides, the declarative macro used in this crate should be easier on compile times. And it comes with no dependencies at all!

## rsdoc
The [rsdoc] crate provides procedural macros to embed [PlantUML] and images in doc coments.
It can be used with code-blocks (similar to aquamarine) or with external files (similar to this crate).
So, in this case, for me it was just a matter of personal taste, both [PlantUML] and [mermaid] are fantastic
opensource projects. But PlantUML is Java... and my plants always die (_even a cactus I once had! How can a cactus die? The thing should not need water!_).

# Disclaimer
Neither this crate nor it's autor have any relation or affiliation with the [mermaid] project, other that being an user of this magnific library.

All the examples in this documentation have been extracted, verbatim or with minor updates, from the [mermaid documentation].

[mermaid]: https://mermaid.js.org/
[include_str]: https://doc.rust-lang.org/std/macro.include_str.html
[mermaid documentation]: https://mermaid.js.org/intro/n00b-gettingStarted.html
[aquamarine]: https://crates.io/crates/aquamarine
[rsdoc]: https://crates.io/crates/rsdoc
[PlantUML]: https://plantuml.com/

License: MIT
