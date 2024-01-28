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
pub fn test_lib(){
    let mut rule_set = RuleSet::new("poke_tile_set.json").unwrap();
    rule_set.ini_expand();
    let sym_d = Symmetry::symmetry_dictionary("symmetry.json").unwrap();
    let result_path = "results/".to_string();
    let new_rules = TileSet::expand(&sym_d,&mut rule_set);

    let n_rule_set = rule_set.expand(&sym_d,&new_rules);

    let tile_set = TileSet::new_r(&n_rule_set);
    let tile_set_size = tile_set.size;

    let mut probabilities = HashMap::new();
    probabilities.insert("c.png".to_string(), 500);
    probabilities.insert("a.png".to_string(), 100);
    probabilities.insert("r1.png".to_string(), 500);
    probabilities.insert("r2.png".to_string(), 100);
    let mut wave = Wave::new(&tile_set, (5,5),probabilities);

    wave.set_pos((0,0), "r2.png".to_string(),&n_rule_set);
    wave.set_pos((4,4), "r2.png".to_string(),&n_rule_set);
    wave.set_pos((0,4), "r2.png".to_string(),&n_rule_set);
    wave.set_pos((4,0), "r2.png".to_string(),&n_rule_set);

    wave.loop_propagate(tile_set_size,&n_rule_set,Some(&result_path));
    render::render_wave(&wave,format!("{}final.png",result_path),&n_rule_set.tiles_path);
}

rustler::init!("Elixir.WfcApp.Rust", [add,test_lib]);
