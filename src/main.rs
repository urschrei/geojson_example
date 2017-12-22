extern crate geojson;
use geojson::{GeoJson, Geometry, Value};

extern crate rayon;
use rayon::prelude::*;

/// Process GeoJSON geometries
fn match_geometry(geom: &Geometry) {
    match geom.value {
        Value::Polygon(_) => println!("Matched a Polygon"),
        Value::MultiPolygon(_) => println!("Matched a MultiPolygon"),
        Value::GeometryCollection(ref gc) => {
            println!("Matched a GeometryCollection");
            // GeometryCollections contain other Geometry types, and can nest
            // we deal with this by recursively processing each geometry
            gc.par_iter().for_each(|geometry| match_geometry(geometry))
        }
        // Point, LineString, and their Multi– counterparts
        _ => println!("Matched some other geometry"),
    }
}

/// Process top-level GeoJSON items
fn process_geojson(gj: &GeoJson) {
    match *gj {
        GeoJson::FeatureCollection(ref ctn) => ctn.features
            // Iterate in parallel when appropriate
            .par_iter()
            // Only pass on non-empty geometries, doing so by reference
            .filter_map(|feature| feature.geometry.as_ref())
            .for_each(|geometry| match_geometry(geometry)),
        GeoJson::Feature(ref feature) => {
            if let Some(ref geom) = feature.geometry {
                match_geometry(geom)
            }
        }
        GeoJson::Geometry(ref geometry) => match_geometry(geometry),
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
    process_geojson(&geojson);
}
