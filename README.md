# Parsing GeoJSON using Rust
This is a minimal example demonstrating GeoJSON parsing using Rust. In order to run it, you'll need at least Rust 1.21. In most cases, the easiest way to do this is using [Rustup](https://rustup.rs).

If you aren't familiar with it, it may be helpful to familiarise yourself with the [GeoJSON spec](https://tools.ietf.org/html/rfc7946), as this should make it obvious why e.g. `Feature` geometries in [`rust-geojson`](https://docs.rs/geojson/0.9.1/geojson/struct.Feature.html) are `Option`.

The example GeoJSON used is deliberately baroque: `GeometryCollection` isn't in wide use, and the use of nested `GeometryCollection`s is discouraged by the spec, and is all but unknown "in the wild". Nevertheless, if we need to e.g. extract all `Polygon` instances in an arbitrary GeoJSON file, we have to be able to process them – this turns out to be relatively painless using a recursive function.

The example code could be more minimal, but this is an ideal use case for [Rayon](https://docs.rs/rayon/) in order to parallelise the processing, so iterators have been substituted for `for { … }` loops to faciliate its use, leading to the requirement for Rust 1.21 or later.

Note that neither of the functions take ownership of the `GeoJSON` struct or clone any part of it — you're free to use `geojson` again as soon as `process_geojson` returns — and the code doesn't allocate, if you care about that sort of thing.

## Further Work
If you want to manipulate GeoJSON objects, you'll most likely want to convert them to [`Geo`](https://docs.rs/geo) types, which provide a wide range of algorithms and methods on geometric objects. To faciliate this,`rust-geojson` provides the `conversion::try_into` trait for this on its `Value` structs.  

This also requires a more involved parsing process, since the conversion consumes the callee – the [`polylabel_cmd`](https://github.com/urschrei/polylabel_cmd) crate contains parsing and conversion code which has the same structure as this example.
