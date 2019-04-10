use std::mem::replace;

use geo::algorithm::convexhull::ConvexHull;
use geo_types::{LineString, Point, Polygon};
use geojson::conversion::TryInto;
use geojson::{GeoJson, Geometry, Value};
use serde_json::to_string_pretty;
use rayon::prelude::*;

/// Process top-level `GeoJSON` items
fn process_geojson(gj: &mut GeoJson) {
    match *gj {
        GeoJson::FeatureCollection(ref mut collection) => collection.features
            .par_iter_mut()
            // Only pass on non-empty geometries
            .filter_map(|feature| feature.geometry.as_mut())
            .for_each(|geometry| process_geometry(geometry)),
        GeoJson::Feature(ref mut feature) => {
            if let Some(ref mut geometry) = feature.geometry {
                process_geometry(geometry)
            }
        }
        GeoJson::Geometry(ref mut geometry) => process_geometry(geometry),
    }
}

/// Process `GeoJSON` Geometries
fn process_geometry(geom: &mut Geometry) {
    match geom.value {
        // Only modify Polygon geometries
        Value::Polygon(_) => calculate_hull(Some(geom)),
        Value::GeometryCollection(ref mut collection) => {
            // GeometryCollections contain other Geometry types, and can nest
            // we deal with this by recursively processing each geometry
            collection
                .par_iter_mut()
                .for_each(|geometry| process_geometry(geometry))
        }
        // Point, LineString, and their Multi– counterparts
        _ => (),
    }
}

/// Modify a Polygon geometry by mutating its shell into its convex hull
fn calculate_hull(geom: Option<&mut Geometry>) {
    if let Some(gmt) = geom {
        // construct a placeholder empty Polygon – this doesn't allocate
        let shell: Vec<Point<f64>> = Vec::new();
        let rings = Vec::new();
        let fake_polygon: Polygon<f64> = Polygon::new(LineString::from(shell), rings);
        // convert it into a Value, and swap it for the actual Polygon
        let intermediate = replace(&mut gmt.value, Value::from(&fake_polygon));
        let mut geo_type: Polygon<f64> = intermediate.try_into().unwrap();
        // modify the borrowed, converted Value
        geo_type = geo_type.convex_hull();
        // and put it back
        gmt.value = Value::from(&geo_type);
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
    let mut geojson = geojson_str.parse::<GeoJson>().unwrap();
    process_geojson(&mut geojson);
    println!("{}", to_string_pretty(&geojson).unwrap());
}
