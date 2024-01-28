use std::collections::HashMap;
use std::fs;
use serde_derive::{Deserialize, Serialize};
use crate::tile_set::AdjacencyRule;

#[derive(Deserialize, Serialize, Debug)]
pub struct Symmetry {
    pub eq: Vec<u32>,
    pub sym: Vec<String>
}

impl Symmetry {
    pub fn symmetry_dictionary(symmetry_json: &str) -> Result<HashMap<String, Symmetry>, std::io::Error> {
        let symmetry_dictionary = {
            let file_path = fs::read_to_string(&symmetry_json)?;
            serde_json::from_str(&file_path).unwrap()
        };
    Ok(symmetry_dictionary)
    }

    pub fn apply_transformations(tile_path: &String, rule: &AdjacencyRule, symmetry_dictionary: &HashMap<String, Symmetry>){
        
        dbg!(&rule.symmetry);
        let eq = &symmetry_dictionary[&rule.symmetry].eq;
        let mut e_tiles = Vec::new();
        e_tiles.push(0 as u32);
        for i in 1..8 {
            let transformation = &eq[i as usize];
            if !(e_tiles.contains(transformation)) {
                Self::transform(tile_path, i);
                e_tiles.push(*transformation);
            } 
        }
        //Self::transform(tile_path,4);
    }

    pub fn transform(tile_path: &String, transformation: u32){
        if let Ok(image) = image::open(tile_path) {
            let transformed_image = match transformation {
                1 => image.rotate90(),
                2 => image.rotate180(),
                3 => image.rotate270(),
                4 => image.fliph(),
                5 => image.fliph().rotate90(),
                6 => image.fliph().rotate180(),
                7 => image.fliph().rotate270(),
                _ => image,
            };
            if let Some(dot_index) = tile_path.rfind('.') {
                let output_path = format!("{}-{}{}", &tile_path[..dot_index], transformation.to_string(), &tile_path[dot_index..]);
                if let Err(err) = transformed_image.save(output_path) {
                    eprintln!("Error saving transformed image: {}", err);
                } else {
                    println!("Image transformed and saved successfully.");
                }
            } else {
                eprintln!("Invalid file name format: {}", tile_path);
            }
        } else {
            eprintln!("Error opening image.");
        }
    }
}