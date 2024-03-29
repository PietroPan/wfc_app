use std::collections::HashMap;
use std::fs;
use serde_derive::{Deserialize, Serialize};
use image;
use crate::tile_set::AdjacencyRule;

#[derive(Deserialize, Serialize, Debug)]
pub struct Symmetry {
    //The index of the these vector represent a transformation Ex: eq[0] is none, sym[3] is rotate270 etc 
    pub eq: Vec<u32>, //For every transformation indicates what is the new tile that tile transforms into ex if eq[3] == eq[7] than applying transformation 3 or 7 results in the same tile
    pub sym: Vec<String> //For every transformation indicates what is the new symmetry that tile transforms into ex if sym[3] == sym[7] than applying transformation 3 or 7 results in the same symmetry
}

impl Symmetry {
    // Create a symmetry dictionary given a jason file
    pub fn symmetry_dictionary(symmetry_json: &str) -> Result<HashMap<String, Symmetry>, std::io::Error> {
        let symmetry_dictionary = {
            let file_path = fs::read_to_string(&symmetry_json)?;
            serde_json::from_str(&file_path).unwrap()
        };
    Ok(symmetry_dictionary)
    }

    // Given a tile depending on the symmetry type create new transformations of that tile
    pub fn apply_transformations(tile_path: &String, _tile: &String, rule: &AdjacencyRule, symmetry_dictionary: &HashMap<String, Symmetry>, _new_rules: &mut HashMap<String,AdjacencyRule>){
        
        dbg!(&rule.symmetry);
        let eq = &symmetry_dictionary[&rule.symmetry].eq;
        //let sym = &symmetry_dictionary[&rule.symmetry].sym;
        let mut e_tiles = Vec::new();
        e_tiles.push(0 as u32);
        for i in 1..8 {
            let transformation = &eq[i as usize];
            if !(e_tiles.contains(transformation)) {
                Self::transform(tile_path, i);
                e_tiles.push(*transformation);
                //Always add new symmetry, I think it's not necessary tho
                //new_rules.insert(RuleSet::nnew_tile_name(tile,*transformation), AdjacencyRule::new_empty(sym[i as usize].to_string()));
            } 
        }
        //dbg!(&symmetry_dictionary[&rule.symmetry].eq);
        //Self::transform(tile_path,4);
    }

    // Transform a tile into one of the 7 transformations
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
            println!("{}",tile_path);
            eprintln!("Error opening image.");
        }
    }
}