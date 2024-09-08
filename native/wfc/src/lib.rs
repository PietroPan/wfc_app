use crate::tile_set::RuleSet;
use crate::tile_set::TileSet;
use crate::symmetry::Symmetry;
use crate::wave::Wave;
use std::collections::HashMap;

pub mod render;
pub mod tile_set;
pub mod wave; 
pub mod symmetry;

#[rustler::nif]
fn add(a: i64, b: i64) -> i64 {
    a + b
}

#[rustler::nif]
// Requires path to json rule set, path to directory with tiles, path to json with symmetry dictionary, size of image, result path to save image, name of image to save, map of probabilities of each tile, starting tiles in wave, new rules to be added after expanding the rule set
pub fn generate_image(rule_set: &str, tile_set: &str, symmetry: &str, (x,y): (i32,i32), results: &str, name: &str, probabilities: HashMap<String,u32>, s_tiles: HashMap<String,String>, n_rules: Vec<&str>) -> String{
    // Creates and expands rule set
    let mut rule_set = RuleSet::new2(rule_set,tile_set.to_string()).unwrap();
    rule_set.ini_expand();
    let sym_d = Symmetry::symmetry_dictionary(symmetry).unwrap();
    let result_path = results.to_string();
    // Creates and expands tile set creating new tiles in necessary
    let new_rules = TileSet::expand(&sym_d,&mut rule_set);

    // If new tiles were created than add them to the rule set and expand again
    let mut n_rule_set = rule_set.expand(&sym_d,&new_rules);

    // Add new rules to the rule set
    for rule in n_rules.clone() {
        let srule: Vec<&str> = rule.split(' ').collect();
        println!("{} {} {}",srule[0],srule[1],srule[2]);
        n_rule_set.add_rule(format!("Q"),srule[2].to_string(),srule[1].parse::<u32>().unwrap(),format!("Q"),srule[0].to_string());
    }

    // If new rules were added, expand rule set again
    let n_n_rule_set = n_rule_set.expand(&sym_d,&HashMap::new());

    // Create new tile set given new tiles created in prior processes
    let tile_set = TileSet::new_r(&n_n_rule_set);
    let tile_set_size = tile_set.size;

    //let mut probabilities = HashMap::new();
    //probabilities.insert("c.png".to_string(), 500);
    //probabilities.insert("a.png".to_string(), 100);
    //probabilities.insert("r1.png".to_string(), 500);
    //probabilities.insert("r2.png".to_string(), 100);

    // Create a new wave and collapse regions with starting tiles
    let mut wave = Wave::new(&tile_set, (x,y),probabilities,&n_n_rule_set);
    for (k,v) in s_tiles.clone() {
        println!("{}",k);
        let pos: i32 = k.parse().unwrap();
        let pos_x = pos % x;
        let pos_y = pos/x;
        wave.set_pos((pos_x,pos_y),v,&n_n_rule_set);
    }

    //wave.set_pos((0,0), "r2.png".to_string(),&n_rule_set);
    //wave.set_pos((4,4), "r2.png".to_string(),&n_rule_set);
    //wave.set_pos((0,4), "r2.png".to_string(),&n_rule_set);
    //wave.set_pos((4,0), "r2.png".to_string(),&n_rule_set);

    // Collapse the wave fully
    wave.loop_propagate(tile_set_size,&n_n_rule_set,Some(&result_path));
    // Renders the final image
    render::render_wave(&wave,format!("{}{}.png",result_path,name),&n_n_rule_set.tiles_path);
    // Return the resulting wave in form of string
    return wave.list_tiles();
}

#[rustler::nif]
// Requires path to json rule set, path to directory with tiles, path to json with symmetry dictionary, size of image, result path to save image, name of image to save, map of probabilities of each tile, starting tiles in wave, new rules to be added after expanding the rule set
pub fn generate_image_i(input_images: &str, (tile_x, tile_y): (u32, u32), (x,y): (i32,i32), tiles_path: &str, results: &str, name: &str, probabilities: HashMap<String,u32>, s_tiles: HashMap<String,String>, n_rules: Vec<&str>, n_tries: i32) -> String{
    let mut rule_set = RuleSet::empty(tiles_path.to_string());

    let mut i = 0;
    let result_path = results.to_string();

    render::get_tiles(input_images.to_string(),tile_x,tile_y,&tiles_path.to_string(),&mut rule_set);
    rule_set.calculate_reach(0);
    
    let tile_set = TileSet::new_r(&rule_set);
    let tile_set_size = tile_set.size;
    let probabilities = HashMap::new();
    
    for _n in 0..n_tries{
        let mut wave = Wave::new(&tile_set, (x,y),probabilities.clone(),&rule_set);
        let res_wave = wave.loop_propagate(tile_set_size,&rule_set,Some(&result_path));
        if res_wave == "invalid_wave" {
            println!("Error: wave malformed")
        } else {
            println!("Found answer on attempt number: {}",i+1);
            render::render_wave(&wave,format!("{}{}.png",result_path,name),&rule_set.tiles_path);
            return wave.list_tiles();
        }
        i = i+1;
        //wave.loop_propagate(tile_set_size,&n_rule_set,None);
        //dbg!(wave);
    }

    if i==n_tries {
        println!("Couldn't find a answer");
    }

    return "".to_string();
//add prob n_rules s_ties
}

rustler::init!("Elixir.WfcApp.Rust", [add,generate_image_i]);