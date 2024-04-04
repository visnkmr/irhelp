fn main() {
    let fileread=fs::read_to_string("../export.geojson").unwrap();
    println!("{:?}",fileread.lines().count());
    let data = &fileread;
    
    let feature_collection: FeatureCollection = serde_json::from_str(data).unwrap();
    let listofstations=feature_collection.features;
    let mut vecofgcoord=Vec::new();
    let mut vecofgtypes=Vec::new();
    for i in listofstations{
        vecofgcoord.push(i.geometry.coordinates);
        if let Some(gtype)= (i.geometry.type_){

            vecofgtypes.push(gtype)
        }
    }
    // vecofgtypes.sort();
    println!("{:#?} {:?}", vecofgcoord.len(),vecofgtypes.len());
    vecofgtypes.dedup();
    println!("{:#?} {:?}", vecofgcoord.len(),vecofgtypes.len());

}
use std::fs;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct FeatureCollection {
    #[serde(rename = "type")]
    type_: Option<String>,
    generator: Option<String>,
    copyright: Option<String>,
    timestamp: Option<String>,
    features: Vec<Feature>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Feature {
    #[serde(rename = "type")]
    type_: Option<String>,
    properties: Properties,
    geometry: Geometry,
    id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Properties {
    #[serde(rename = "@id")]
    id: Option<String>,
    internet_access: Option<String>,
    name: Option<String>,
    #[serde(rename = "name:hi")]
    name_hi: Option<String>,
    #[serde(rename = "name:ja")]
    name_ja: Option<String>,
    #[serde(rename = "name:mr")]
    name_mr: Option<String>,
    network: Option<String>,
    operator: Option<String>,
    public_transport: Option<String>,
    railway: Option<String>,
    #[serde(rename = "ref")]
    ref_: Option<String>,
    train: Option<String>,
    wikidata: Option<String>,
    wikipedia: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Geometry {
    #[serde(rename = "type")]
    type_: Option<String>,
    coordinates: Vec<f64>,
}