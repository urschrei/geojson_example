# Parsing GeoJSON using Rust
These are three minimal examples demonstrating GeoJSON parsing using Rust. In order to run them, you'll need at least Stable Rust 1.21. In most cases, the easiest way to do this is using [Rustup](https://rustup.rs).

If you aren't familiar with it, it may be helpful to familiarise yourself with the [GeoJSON spec](https://tools.ietf.org/html/rfc7946), as this should make it obvious why e.g. `Feature` geometries in [`rust-geojson`](https://docs.rs/geojson/0.9.1/geojson/struct.Feature.html) are `Option`.

The example GeoJSON used is deliberately baroque: `GeometryCollection` isn't in wide use, and the use of nested `GeometryCollection`s is discouraged by the spec, being all but unknown "in the wild". Nevertheless, if we need to e.g. extract all `Polygon` objects in an arbitrary GeoJSON file, we have to be able to process them – this turns out to be relatively painless using a recursive function.

The example code could be more minimal, but this is an ideal use case for [Rayon](https://docs.rs/rayon/) in order to parallelise the processing, so iterators have been substituted for `for { … }` loops to faciliate its use, leading to the requirement for Rust 1.21 or later. If you'd prefer to use `for` loops and avoid iterators, the [`plain`](https://github.com/urschrei/geojson_example/tree/plain) branch is available.

## Approach
Three different approaches to parsing GeoJSON are shown:
1. [`borrowed.rs`](src/borrowed.rs) shows parsing using only borrowed data, and does not consume the GeoJSON, clone any part of it, or allocate – you're free to use `geojson` again as soon as `process_geojson` returns. Run it using cargo: `cargo run --bin borrowed`
2. [`owned.rs`](src/owned.rs) shows parsing and conversion to [`Geo`](https://docs.rs/geo) types, which necessarily consumes the GeoJSON, as `Geo`'s primitives currently require owned data. To faciliate conversions of this kind,`rust-geojson` provides the `conversion::try_into` trait for this on its `Value` structs. This approach is the most flexible, especially if you need to add or remove data, as opposed to modifying it. Run it using cargo: `cargo run --bin owned`
3. [`borrowed_modify.rs`](src/borrowed_modify.rs) (Run it using cargo: `cargo run --bin borrowed_modify`) shows parsing and modification of geometries using only mutably borrowed data. The core idea can be seen in `calculate_hull()`: ordinarily, `take()` could be used to remove the `T` from an `Option<T>` – in this case a `Geometry` – convert its `value` to a `Geo` type, and modify it before replacing it. However, this approach will only work for the `Feature` type; input which contains top-level `Geometry` and/or `GeometryCollection` types can't be modified in this way, due to the lack of `Option`. However, a more general approach involving [`std::mem::replace`](https://doc.rust-lang.org/std/mem/fn.replace.html), is possible:
    1. We match against `&mut Geometry` (which all [`GeoJson` enum](https://docs.rs/geojson/0.9.1/geojson/enum.GeoJson.html) variants are guaranteed to have) on its `value`, wrapping *that* in an `Option`
    2. This is passed to the conversion function
    3. In the conversion function, a non-allocating `Geo` placeholder geometry is constructed, converted into a `Value`, and swapped for the `Value` we wish to modify, using `replace`. The original geometry can then be modified or freely used, before being swapped back into the borrowed `Geometry.value`, before the function returns.

## Further Work
The [`polylabel_cmd`](https://github.com/urschrei/polylabel_cmd) crate contains more advanced parsing and conversion code which has the same structure as these examples.

## License
[BSD Zero Clause License](LICENSE)

