extern crate geojson;
use geojson::{GeoJson, Geometry, Value};
use geojson::conversion::TryInto;

extern crate rayon;
use rayon::prelude::*;

extern crate geo;
use geo::{Polygon};
use geo::algorithm::centroid::Centroid;

/// Process GeoJSON geometries
fn match_geometry(geom: Geometry) {
    match geom.value {
        Value::Polygon(_) => {
            let poly: Polygon<f64> = geom.value.try_into().expect("Unable to convert Polygon");
            let centroid = poly.centroid().unwrap();
            println!("Found a polygon. Its centroid is ({}, {})", centroid.x(), centroid.y());

        },
        Value::MultiPolygon(_) => println!("Matched a MultiPolygon"),
        Value::GeometryCollection(collection) => {
            println!("Matched a GeometryCollection");
            // GeometryCollections contain other Geometry types, and can nest
            // we deal with this by recursively processing each geometry
            collection
                .into_par_iter()
                .for_each(match_geometry)
        }
        // Point, LineString, and their Multi– counterparts
        _ => println!("Matched some other geometry"),
    }
}

/// Process top-level GeoJSON items
fn process_geojson(gj: GeoJson) {
    match gj {
        GeoJson::FeatureCollection(collection) => collection.features
            // Iterate in parallel  appropriate
            .into_par_iter()
            // Only pass on non-empty geometries, doing so by reference
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
    let geojson_str = r#"
    {
      "type": "GeometryCollection",
      "geometries": [
        {"type": "Point", "coordinates": [0,1]},
        {"type": "MultiPoint", "coordinates": [[-1,0],[1,0]]},
        {"type": "LineString", "coordinates": [[-1,-1],[1,-1]]},
        {"type": "MultiLineString", "coordinates": [
          [[-2,-2],[2,-2]],
          [[-3,-3],[3,-3]]
        ]},
        {"type": "Polygon", "coordinates": [
          [[-5,-5],[5,-5],[0,5],[-5,-5]],
          [[-4,-4],[4,-4],[0,4],[-4,-4]]
        ]},
        { "type": "MultiPolygon", "coordinates": [[
          [[-7,-7],[7,-7],[0,7],[-7,-7]],
          [[-6,-6],[6,-6],[0,6],[-6,-6]]
        ],[
          [[-9,-9],[9,-9],[0,9],[-9,-9]],
          [[-8,-8],[8,-8],[0,8],[-8,-8]]]
        ]},
        {"type": "GeometryCollection", "geometries": [
          {"type": "Polygon", "coordinates": [
            [[-5.5,-5.5],[5,-5],[0,5],[-5,-5]],
            [[-4,-4],[4,-4],[0,4],[-4.5,-4.5]]
          ]}
        ]}
      ]
    }
    "#;
    let geojson = geojson_str.parse::<GeoJson>().unwrap();
    process_geojson(geojson);
}
