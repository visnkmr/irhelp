// #[derive(Clone,Serialize,Deserialize,Debug)]
// struct statdist{
//     name:String,
//     distance:f64
// }
#[derive(Clone,Serialize,Deserialize,Debug)]
struct statdist{
        name:String,
    xycoord:String,
    distance:f64
}
fn convert_spherical_to_cartesian(latitude: f64, longitude: f64, radius: f64) -> (f64, f64) {
    // Convert from Degrees to Radians
    let lat_rad = latitude * std::f64::consts::PI / 180.0;
    let lon_rad = longitude * std::f64::consts::PI / 180.0;

    // Calculate x and y
    let x = radius * lat_rad.cos() * lon_rad.cos();
    let y = radius * lat_rad.cos() * lon_rad.sin();

    (x.round(), y.round())
    // (latitude.round(),longitude.round())
}
fn main() {
    fs::write("../test.txt", format!("{:?}","")).unwrap();
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
            if let Some(mut sname)= (i.properties.name){
                if let Some(mut altname)= (i.properties.alt_name){
                    sname=format!("{},{}",sname,altname);

                }if let Some(mut oldname)= (i.properties.old_name){
                    sname=format!("{},{}",sname,oldname);

                }
                // println!("{sname}");
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
    // let mut vvecofdist= Vec::new();
    let found:Vec<usize>=vecofstationnames.iter().enumerate().filter_map(|(l,f)|if(f.to_lowercase().contains("")){Some(l)}else{None}).collect();
    println!("{:?}",found);
    let ci=found[0];
    let i=vecofgcoord[ci].clone();
    // for (ci,i) in vecofgcoord.clone().iter().enumerate()
     {
        let mut vecofdist= Vec::new();
        let source=Point::new(i[0],i[1]);
        for (index,j) in vecofgcoord.clone().iter().enumerate() {
            let dest=Point::new(j[0],j[1]);
            let distance = source.haversine_distance(&dest);
            vecofdist.push(
            statdist{
                // name:format!("from {} to {}",vecofstationnames[found[0]],vecofstationnames[index]),
                name:format!("{:?}",vecofstationnames[index]),
                xycoord:format!("{:?}",((convert_spherical_to_cartesian(j[0],j[1], 6367.)).0-(convert_spherical_to_cartesian(i[0],i[1], 6367.)).0,convert_spherical_to_cartesian(j[0],j[1], 6367.).1-(convert_spherical_to_cartesian(i[0],i[1], 6367.)).1)),
                distance:
                (distance*0.001).round()
            }
        )
        }
        println!("{ci}");
        fs::write(format!("./selected.txt"), serde_json::to_string(&vecofdist).unwrap());
        // vvecofdist.push(vecofdist)
    }
    println!("finished computation");
//     vecofdist.sort_by(|a,b|if(a.distance>b.distance){
//         Ordering::Greater
//     }
//     else{
//         Ordering::Less
//     }
// );
    // let mut sb=String::new();
    // for i in vvecofdist.clone(){
    //     sb=(format!("{} , {}",sb,serde_json::to_string(&i).unwrap()))
    // }
    // println!("{:?}",sb.len());
    // println!("{:?}",vvecofdist.len());
    // let mut f = File::options().write(true).append(true).create(true).open("../test.txt").unwrap();
    // let mut lw=LineWriter::new(f);
    // fs::write("../test.txt", serde_json::to_string(&vvecofdist).unwrap());
    // for (index,i) in (vecofdist.iter().enumerate()){
    //     lw.write_all(&format!("{}",serde_json::to_string(&i).unwrap()).as_bytes());
    //     // if(index%vecofdist.len()==0){

    //         lw.write_all(b"\n");
    //     // }
    //     // else{
    //     //     lw.write_all(b" , ");

    //     // }
    // }
    // fs::write("../test.txt", format!("{:?}",vecofdist)).unwrap();
    // let result: Vec<String> = vecofstationnames.iter().flat_map(|x| vec![x.clone(); vecofstationnames.len()]).collect();
    // vecofgtypes.sort();
    // vecofgtypes.dedup();
    // println!("{:#?} {:?} {:?}", vecofgcoord.len(), vecofstationnames.len(),vecofgtypes.len());
    // println!("{:#?}", result.len());

}
use core::time;
use std::{cmp::Ordering, fs::{self, File}, io::{LineWriter, Write}, vec};

use geo::{HaversineDistance, Point};
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
    alt_name: Option<String>,
    int_name: Option<String>,
    internet_access: Option<String>,
    name: Option<String>,
    #[serde(rename = "name:hi")]
    name_hi: Option<String>,
    #[serde(rename = "name:ja")]
    name_ja: Option<String>,
    #[serde(rename = "name:mr")]
    name_mr: Option<String>,
    network: Option<String>,
    old_name: Option<String>,
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