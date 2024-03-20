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
pub fn generate_image(rule_set: &str, tile_set: &str, symmetry: &str, (x,y): (i32,i32), results: &str, name: &str, probabilities: HashMap<String,u32>, s_tiles: HashMap<String,String>) -> String{
    let mut rule_set = RuleSet::new2(rule_set,tile_set.to_string()).unwrap();
    rule_set.ini_expand();
    let sym_d = Symmetry::symmetry_dictionary(symmetry).unwrap();
    let result_path = results.to_string();
    let new_rules = TileSet::expand(&sym_d,&mut rule_set);

    let n_rule_set = rule_set.expand(&sym_d,&new_rules);

    let tile_set = TileSet::new_r(&n_rule_set);
    let tile_set_size = tile_set.size;

    //let mut probabilities = HashMap::new();
    //probabilities.insert("c.png".to_string(), 500);
    //probabilities.insert("a.png".to_string(), 100);
    //probabilities.insert("r1.png".to_string(), 500);
    //probabilities.insert("r2.png".to_string(), 100);

    let mut wave = Wave::new(&tile_set, (x,y),probabilities);

    for (k,v) in s_tiles.clone() {
        println!("{}: {}",k,v);
        let pos: i32 = k.parse().unwrap();
        let pos_x = pos % x;
        let pos_y = pos-pos_x*x;
        wave.set_pos((pos_x,pos_y),v,&n_rule_set);
    }

    //wave.set_pos((0,0), "r2.png".to_string(),&n_rule_set);
    //wave.set_pos((4,4), "r2.png".to_string(),&n_rule_set);
    //wave.set_pos((0,4), "r2.png".to_string(),&n_rule_set);
    //wave.set_pos((4,0), "r2.png".to_string(),&n_rule_set);

    wave.loop_propagate(tile_set_size,&n_rule_set,Some(&result_path));
    render::render_wave(&wave,format!("{}{}.png",result_path,name),&n_rule_set.tiles_path);
    return wave.list_tiles();
}

rustler::init!("Elixir.WfcApp.Rust", [add,generate_image]);
