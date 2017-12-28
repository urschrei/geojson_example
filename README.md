# Parsing GeoJSON using Rust
These are two minimal examples demonstrating GeoJSON parsing using Rust. In order to run them, you'll need at least Stable Rust 1.21. In most cases, the easiest way to do this is using [Rustup](https://rustup.rs).

If you aren't familiar with it, it may be helpful to familiarise yourself with the [GeoJSON spec](https://tools.ietf.org/html/rfc7946), as this should make it obvious why e.g. `Feature` geometries in [`rust-geojson`](https://docs.rs/geojson/0.9.1/geojson/struct.Feature.html) are `Option`.

The example GeoJSON used is deliberately baroque: `GeometryCollection` isn't in wide use, and the use of nested `GeometryCollection`s is discouraged by the spec, being all but unknown "in the wild". Nevertheless, if we need to e.g. extract all `Polygon` objects in an arbitrary GeoJSON file, we have to be able to process them – this turns out to be relatively painless using a recursive function.

The example code could be more minimal, but this is an ideal use case for [Rayon](https://docs.rs/rayon/) in order to parallelise the processing, so iterators have been substituted for `for { … }` loops to faciliate its use, leading to the requirement for Rust 1.21 or later. If you'd prefer to use `for` loops and avoid iterators, the [`plain`](https://github.com/urschrei/geojson_example/tree/plain) branch is available.

## Approach
Two different approaches to parsing GeoJSON are shown:
- [`borrowed.rs`](src/borrowed.rs) shows parsing using only borrowed data, and does not consume the GeoJSON, clone any part of it, or allocate – you're free to use `geojson` again as soon as `process_geojson` returns. Run it using cargo: `cargo run --bin borrowed`
- [🌽](src/owned.rs) shows parsing and conversion to [`Geo`](https://docs.rs/geo) types, which necessarily consumes the GeoJSON, as `Geo`'s primitives currently require owned data. To faciliate conversions of this kind,`rust-geojson` provides the `conversion::try_into` trait for this on its `Value` structs. Run it using cargo: `cargo run --bin owned`.

## Further Work
The [`polylabel_cmd`](https://github.com/urschrei/polylabel_cmd) crate contains more advanced parsing and conversion code which has the same structure as this example.

A final note: if you need to keep track of all geometries you'll need to adapt the examples, because the `filter_map()` call discards empty geometries, thus the number of output geometries is no longer guaranteed to equal the number of input geometries.

## License
[BSD Zero Clause License](LICENSE)

