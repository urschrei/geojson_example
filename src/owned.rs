use geo::algorithm::centroid::Centroid;
use geo_types::Polygon;
use geojson::{GeoJson, Geometry, Value};
use std::convert::TryInto;

/// Process GeoJSON geometries
fn match_geometry(geom: Geometry) {
    match geom.value {
        Value::Polygon(_) => {
            let poly: Polygon<f64> = geom.value.try_into().expect("Unable to convert Polygon");
            let centroid = poly.centroid().unwrap();
            println!(
                "Matched a Polygon with centroid ({}, {})",
                centroid.x(),
                centroid.y()
            );
        }
        Value::MultiPolygon(_) => println!("Matched a MultiPolygon"),
        Value::GeometryCollection(collection) => {
            println!("Matched a GeometryCollection");
            // GeometryCollections contain other Geometry types, and can nest
            // we deal with this by recursively processing each geometry
            collection.into_iter().for_each(match_geometry)
        }
        // Point, LineString, and their Multi– counterparts
        _ => println!("Matched some other geometry"),
    }
}

/// Process top-level GeoJSON items
fn process_geojson(gj: GeoJson) {
    match gj {
        GeoJson::FeatureCollection(collection) => collection
            .features
            // Iterate in parallel where appropriate
            .into_iter()
            // Only pass on non-empty geometries
            .filter_map(|feature| feature.geometry)
            .for_each(match_geometry),
        GeoJson::Feature(feature) => {
            if let Some(geometry) = feature.geometry {
                match_geometry(geometry)
            }
        }
        GeoJson::Geometry(geometry) => match_geometry(geometry),
    }
}

fn main() {
    let geojson_str = include!("test.geojson");
    let geojson = geojson_str.parse::<GeoJson>().unwrap();
    process_geojson(geojson);
}
