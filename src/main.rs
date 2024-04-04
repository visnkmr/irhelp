fn main() {
    fs::write("../test.txt", format!("{:?}","vecofdist")).unwrap();

    let fileread=fs::read_to_string("../export.geojson").unwrap();
    // println!("{:?}",fileread.lines().count());
    let data = &fileread;
    
    let feature_collection: FeatureCollection = serde_json::from_str(data).unwrap();
    let listofstations=feature_collection.features;
    let mut vecofgcoord=Vec::new();
    let mut vecofgtypes=Vec::new();
    let mut vecofstationnames=Vec::new();
    for i in listofstations{
        if(i.geometry.type_.is_some() && i.properties.name.is_some()){
            if let Some(gtype)= (i.geometry.type_){
    
                vecofgtypes.push(gtype)
            }
            if let Some(sname)= (i.properties.name){
    
                vecofstationnames.push(sname)
            }
            vecofgcoord.push(i.geometry.coordinates);
        }
        else{
            // println!("{:?}",(i.properties));
        }
    }
    // for i in 0..vecofstationnames.len()-1{
    //     println!("{:?}",vecofgcoord[i]);
    // }
    let mut vecofdist= Vec::new();
    let i=vecofgcoord[0].clone();
    for i in vecofgcoord.clone()
     {
        let source=Location::new(i[0],i[1]);
        for j in vecofgcoord.clone() {
            let dest=Location::new(j[0],j[1]);
            let distance = source.haversine_distance_to(&dest);
            vecofdist.push(distance.meters()*0.001)
        }
    }
    println!("{:?}",vecofdist.len());
    fs::write("../test.txt", format!("{:?}",vecofdist)).unwrap();
    // let result: Vec<String> = vecofstationnames.iter().flat_map(|x| vec![x.clone(); vecofstationnames.len()]).collect();
    // vecofgtypes.sort();
    vecofgtypes.dedup();
    println!("{:#?} {:?} {:?}", vecofgcoord.len(), vecofstationnames.len(),vecofgtypes.len());
    // println!("{:#?}", result.len());

}
use core::time;
use std::{fs, vec};

use geoutils::Location;
use serde::{de, Deserialize, Serialize};

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