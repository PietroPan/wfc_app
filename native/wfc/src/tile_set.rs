use std::fs;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::symmetry::Symmetry;
use std::fs::File;

#[derive(Debug)]
pub struct TileSet {
    pub tiles: Vec<String>, // List of tiles
    pub tiles_path: String, // Path to the folder with those tiles
    pub size: u32 // Number of tiles in the list
}

impl TileSet {
    // Given a path to a directory with tiles creates a tile set with all images inside that folder
    pub fn new(tiles_path: &str) -> TileSet {
        let tiles:Vec<String> = fs::read_dir(tiles_path).unwrap()
            .map(|x| x.unwrap().file_name().into_string().unwrap())
            .filter(|x| x.ends_with(".jpg") || x.ends_with(".png"))
            .collect();
        let tiles_path = tiles_path.clone().to_string();
        TileSet {
            size: tiles.len() as u32,
            tiles,
            tiles_path
        }
    }

    // Given a rule set creates a tile set with all of the tiles and tile path listed in that rule set
    pub fn new_r(rule_set: &RuleSet) -> TileSet {
        let tiles:Vec<String> = rule_set.adjacency_rules.keys().cloned().collect();
        TileSet {
            size: tiles.len() as u32,
            tiles,
            tiles_path: rule_set.tiles_path.clone()
        }
    }

    // Looks at all of the tiles in a rule set and creates new tiles from them given their symmetry and a symmetry dictionary
    // Returns a new Adjacency rule for every new tile created
    pub fn expand(symmetry_dictionary: &HashMap<String, Symmetry>, rule_set:&mut RuleSet) -> HashMap<String, AdjacencyRule> {

        let mut new_rules = HashMap::new();
        
        for (tile,rules) in rule_set.clone().adjacency_rules {
            let tile_path = format!("{}{}", rule_set.tiles_path, tile);
            Symmetry::apply_transformations(&tile_path, &tile, &rules, symmetry_dictionary,&mut new_rules);
            let symmetry = rules.symmetry.as_str();
            match symmetry{
                "B" => {
                    if let Some(rule) = rule_set.adjacency_rules.get_mut(&tile){
                        rule.left_tiles.push(RuleSet::new_tile_name(&tile, 2,"F"));
                        rule.right_tiles.push(RuleSet::new_tile_name(&tile, 2,"F"));
                        new_rules.insert(RuleSet::new_tile_name(&tile, 2,"F"), AdjacencyRule::new_empty("B".to_string()));
                        rule.up_tiles.push(tile);                    }
                },
                "T" => {
                    if let Some(rule) = rule_set.adjacency_rules.get_mut(&tile){
                        rule.up_tiles.push(RuleSet::new_tile_name(&tile, 2,"F"));
                        rule.down_tiles.push(RuleSet::new_tile_name(&tile, 2,"F"));
                        new_rules.insert(RuleSet::new_tile_name(&tile, 2,"F"), AdjacencyRule::new_empty("T".to_string()));
                        rule.left_tiles.push(tile);
                    }
                },
                "Q" => {
                    if let Some(rule) = rule_set.adjacency_rules.get_mut(&tile){
                        rule.up_tiles.push(RuleSet::new_tile_name(&tile, 3,"F"));
                        rule.down_tiles.push(RuleSet::new_tile_name(&tile, 3,"F"));
                        new_rules.insert(RuleSet::new_tile_name(&tile, 3,"F"), AdjacencyRule::new_empty("L".to_string()));
                    }
                },
                "L" => {
                    if let Some(rule) = rule_set.adjacency_rules.get_mut(&tile){
                        rule.up_tiles.push(RuleSet::new_tile_name(&tile, 1,"F"));
                        rule.down_tiles.push(RuleSet::new_tile_name(&tile, 1,"F"));
                        new_rules.insert(RuleSet::new_tile_name(&tile, 1,"F"), AdjacencyRule::new_empty("Q".to_string()));
                    }
                },
                "I" => {
                    if let Some(rule) = rule_set.adjacency_rules.get_mut(&tile){
                        rule.up_tiles.push(tile.clone());
                        rule.left_tiles.push(tile);
                    }
                },
                "Z" => {
                    if let Some(rule) = rule_set.adjacency_rules.get_mut(&tile){
                        rule.up_tiles.push(RuleSet::new_tile_name(&tile, 1,"F"));
                        new_rules.insert(RuleSet::new_tile_name(&tile, 1,"F"), AdjacencyRule::new_empty("Z".to_string()));
                    }
                },
                "S" => {
                    if let Some(rule) = rule_set.adjacency_rules.get_mut(&tile){
                        rule.up_tiles.push(RuleSet::new_tile_name(&tile, 4,"F"));
                        rule.left_tiles.push(RuleSet::new_tile_name(&tile, 4,"F"));
                        new_rules.insert(RuleSet::new_tile_name(&tile, 4,"F"), AdjacencyRule::new_empty("S".to_string()));
                    }
                },
                "X" => {
                    if let Some(rule) = rule_set.adjacency_rules.get_mut(&tile){
                        rule.up_tiles.push(tile);
                    }
                },
                _ => println!("OI")
            }
        }
        return new_rules;
    }
}


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AdjacencyRule {
    pub symmetry: String, // Symmetry of Tile
    pub up_tiles: Vec<String>, // Possible tiles up from that tile
    pub right_tiles: Vec<String>, // Possible tiles right from that tile
    pub down_tiles: Vec<String>, // Possible tiles down from that tile
    pub left_tiles: Vec<String> // Possible tiles left from that tile
}

impl AdjacencyRule{
    // Creates a adjacency rule given all of the fields
    pub fn new(symmetry: String, up_tiles: Vec<String>, right_tiles: Vec<String>, down_tiles: Vec<String>, left_tiles: Vec<String>,) -> AdjacencyRule {
        AdjacencyRule {
            symmetry: symmetry,
            up_tiles: up_tiles,
            right_tiles: right_tiles,
            down_tiles: down_tiles,
            left_tiles: left_tiles,
        }
    }

    // Creates a empty adjacency rule
    pub fn new_empty(symmetry: String) -> AdjacencyRule {
        AdjacencyRule {
            symmetry: symmetry,
            up_tiles: Vec::new(),
            right_tiles: Vec::new(),
            down_tiles: Vec::new(),
            left_tiles: Vec::new(),
        }
    }

    // Creates a simple adjacency rule with a single value 
    pub fn new_rule(symmetry: String,dir: u32, tile: String) -> AdjacencyRule {
        let mut up_tiles = Vec::new();
        let mut right_tiles = Vec::new();
        let mut down_tiles = Vec::new();
        let mut left_tiles = Vec::new();
        match dir{
            0 => up_tiles.push(tile),
            1 => right_tiles.push(tile),
            2 => down_tiles.push(tile),
            3 => left_tiles.push(tile),
            _ => println!("Invalid direction"),
        }
        AdjacencyRule {
            symmetry: symmetry,
            up_tiles: up_tiles,
            right_tiles: right_tiles,
            down_tiles: down_tiles,
            left_tiles: left_tiles,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RuleSet {
    pub tiles_path: String, // Path to the directory with the tiles
    pub adjacency_rules: HashMap<String, AdjacencyRule> // Map that assigns adjacency rules to every tile
}

impl RuleSet {
    // Creates a new rule set given a json file
    pub fn new(rules_json: &str) -> Result<RuleSet, std::io::Error> {
        let rule_set = {
            let file_path = fs::read_to_string(&rules_json)?;
            serde_json::from_str::<RuleSet>(&file_path).unwrap()
        };
    Ok(rule_set)
    }

    // Creates a new rule set given a json file and a path to the tiles directory
    pub fn new2(rules_json: &str, tiles_path: String) -> Result<RuleSet, std::io::Error> {
        let rule_set = {
            let file_path = fs::read_to_string(&rules_json)?;
            serde_json::from_str::<RuleSet>(&file_path).unwrap()
        };
        let adjacency_rules = rule_set.adjacency_rules;
    Ok(RuleSet{adjacency_rules, tiles_path})
    }

    // Transforms a RuleSet structure into a json file
    pub fn to_json(&self, new_json: &str) {
        let file = File::options().write(true).create(true).open(new_json).expect("Unable to open file");
        serde_json::to_writer(file, &self).unwrap();
        //let ruleset = serde_json::to_string(&self).unwrap();
        //fs::write(new_json, ruleset).expect("Unable to write");
    }

    // Adds a adjacency rule and it's symmetric counter part to the rule set
    // The symmetries of the tiles are required in case the tile is not yet in the rule set
    pub fn add_rule(&mut self, sym1: String, tile1: String,direction: u32, sym2:String, tile2: String){
        self.add_rule_aux(sym1, tile1.clone(), direction, tile2.clone());
        self.add_rule_aux(sym2, tile2, (direction+2)%4, tile1);
    }

    // Adds a adjacency rule to the rule set 
    pub fn add_rule_aux(&mut self, symmetry: String, tile1: String,direction: u32, tile2: String){
        if self.adjacency_rules.contains_key(&tile1){
            if let Some(rule) = self.adjacency_rules.get_mut(&tile1){
                match direction {
                    0 => if !rule.up_tiles.contains(&tile2) {rule.up_tiles.push(tile2)},
                    1 => if !rule.right_tiles.contains(&tile2) {rule.right_tiles.push(tile2)},
                    2 => if !rule.down_tiles.contains(&tile2) {rule.down_tiles.push(tile2)},
                    3 => if !rule.left_tiles.contains(&tile2) {rule.left_tiles.push(tile2)},
                    _ => println!("Invalid direction"),
                }
            }
        } else {
            println!("heeeeeeeeeeeeeeereeeeeeeeeeee");
            self.adjacency_rules.insert(tile1, AdjacencyRule::new_rule(symmetry, direction, tile2));
        }
    }

    // For every adjacency rule in a rule set adds it's symmetry counter part to the rule set
    pub fn ini_expand(&mut self){
        for (tile1, rules) in &self.adjacency_rules.clone() {
            for tile2 in &rules.up_tiles{
                RuleSet::add_rule_aux(self, format!("N"), tile2.clone(), 2, tile1.clone());
            }
            for tile2 in &rules.right_tiles{
                RuleSet::add_rule_aux(self, format!("N"), tile2.clone(), 3, tile1.clone());
            }
            for tile2 in &rules.down_tiles{
                RuleSet::add_rule_aux(self, format!("N"), tile2.clone(), 0, tile1.clone());
            }
            for tile2 in &rules.left_tiles{
                RuleSet::add_rule_aux(self, format!("N"), tile2.clone(), 1, tile1.clone());
            }
        }
    }

    // 
    pub fn expand(&mut self, symmetry_dictionary: &HashMap<String, Symmetry>, new_rules: &HashMap<String, AdjacencyRule>) -> RuleSet{
        //let mut n_adjancecy_rules = HashMap::new();
        let old_rules = self.clone();

        //Add new rules to the rule set
        for (tile, rule) in new_rules{
            let _ = &self.adjacency_rules.insert(tile.clone(), rule.clone());
        }

        // Transform every rule in a rule set depending on the symmetry of the tiles involved
        for (tile, rules) in &old_rules.adjacency_rules {
            let mut n_up_tiles = Vec::new();
            let mut n_left_tiles = Vec::new();
            let mut n_down_tiles = Vec::new();
            let mut n_right_tiles = Vec::new();
            // Rotate90
            let new_t = symmetry_dictionary[&rules.symmetry].eq[1];
            let new_s = &symmetry_dictionary[&rules.symmetry].sym[1];
            let tile_name = Self::new_tile_name(tile,new_t,&new_s);
            if *new_s!="@".to_string() {
                for tile in &rules.up_tiles {
                    let nnew_s = &self.adjacency_rules[tile].symmetry;
                    let nnew_t = symmetry_dictionary[nnew_s].eq[1];
                    if *nnew_s!="@".to_string(){
                        let ntile_name = Self::new_tile_name(tile,nnew_t,nnew_s);
                        n_right_tiles.push(ntile_name);
                    }
                    //let nnew_s = &symmetry_dictionary[&self.adjacency_rules[tile].symmetry].sym[1];
                }
                for tile in &rules.right_tiles {
                    let nnew_s = &self.adjacency_rules[tile].symmetry;
                    let nnew_t = symmetry_dictionary[nnew_s].eq[1];
                    if *nnew_s!="@".to_string(){
                        let ntile_name = Self::new_tile_name(tile,nnew_t,nnew_s);
                        n_down_tiles.push(ntile_name);
                    }
                }
                for tile in &rules.down_tiles {
                    let nnew_s = &self.adjacency_rules[tile].symmetry;
                    let nnew_t = symmetry_dictionary[nnew_s].eq[1];
                    if *nnew_s!="@".to_string(){
                        let ntile_name = Self::new_tile_name(tile,nnew_t,nnew_s);
                        n_left_tiles.push(ntile_name);
                    }
                }
                for tile in &rules.left_tiles {
                    let nnew_s = &self.adjacency_rules[tile].symmetry;
                    let nnew_t = symmetry_dictionary[nnew_s].eq[1];
                    if *nnew_s!="@".to_string(){
                        let ntile_name = Self::new_tile_name(tile,nnew_t,nnew_s);
                        n_up_tiles.push(ntile_name);
                    }
                }
                let n_rule = AdjacencyRule::new(new_s.clone(),n_up_tiles.clone(),n_right_tiles.clone(),n_down_tiles.clone(),n_left_tiles.clone());
                //n_adjancecy_rules.insert(tile_name, n_rule);
                if self.adjacency_rules.contains_key(&tile_name){
                    if let Some(rule) = self.adjacency_rules.get_mut(&tile_name){
                        for i in n_down_tiles.clone() {
                            if !rule.down_tiles.contains(&i){
                                rule.down_tiles.push(i);
                            }
                        }
                        for i in n_left_tiles.clone() {
                            if !rule.left_tiles.contains(&i){
                                rule.left_tiles.push(i);
                            }
                        }
                        for i in n_up_tiles.clone() {
                            if !rule.up_tiles.contains(&i){
                                rule.up_tiles.push(i);
                            }
                        }
                        for i in n_right_tiles.clone() {
                            if !rule.right_tiles.contains(&i){
                                rule.right_tiles.push(i);
                            }
                        }
                    }
                }  else {
                    self.adjacency_rules.insert(tile_name, n_rule);
                }
                n_up_tiles.clear();
                n_left_tiles.clear();
                n_down_tiles.clear();
                n_right_tiles.clear();
                
                // Rotate180
                let new_t = symmetry_dictionary[&rules.symmetry].eq[2];
                let new_s = &symmetry_dictionary[&rules.symmetry].sym[2];
                let tile_name = Self::new_tile_name(tile,new_t,&new_s);
                for tile in &rules.up_tiles {
                    let nnew_s = &self.adjacency_rules[tile].symmetry;
                    let nnew_t = symmetry_dictionary[nnew_s].eq[2];
                    if *nnew_s!="@".to_string(){
                        let ntile_name = Self::new_tile_name(tile,nnew_t,nnew_s);
                        n_down_tiles.push(ntile_name);
                    }
                }
                for tile in &rules.right_tiles {
                    let nnew_s = &self.adjacency_rules[tile].symmetry;
                    let nnew_t = symmetry_dictionary[nnew_s].eq[2];
                    if *nnew_s!="@".to_string(){
                        let ntile_name = Self::new_tile_name(tile,nnew_t,nnew_s);
                        n_left_tiles.push(ntile_name);
                    }
                }
                for tile in &rules.down_tiles {
                    let nnew_s = &self.adjacency_rules[tile].symmetry;
                    let nnew_t = symmetry_dictionary[nnew_s].eq[2];
                    if *nnew_s!="@".to_string(){
                        let ntile_name = Self::new_tile_name(tile,nnew_t,nnew_s);
                        n_up_tiles.push(ntile_name);
                    }
                }
                for tile in &rules.left_tiles {
                    let nnew_s = &self.adjacency_rules[tile].symmetry;
                    let nnew_t = symmetry_dictionary[nnew_s].eq[2];
                    if *nnew_s!="@".to_string(){
                        let ntile_name = Self::new_tile_name(tile,nnew_t,nnew_s);
                        n_right_tiles.push(ntile_name);
                    }
                }
                let n_rule = AdjacencyRule::new(new_s.clone(),n_up_tiles.clone(),n_right_tiles.clone(),n_down_tiles.clone(),n_left_tiles.clone());
                //n_adjancecy_rules.insert(tile_name, n_rule);
                if self.adjacency_rules.contains_key(&tile_name){
                    if let Some(rule) = self.adjacency_rules.get_mut(&tile_name){
                        for i in n_down_tiles.clone() {
                            if !rule.down_tiles.contains(&i){
                                rule.down_tiles.push(i);
                            }
                        }
                        for i in n_left_tiles.clone() {
                            if !rule.left_tiles.contains(&i){
                                rule.left_tiles.push(i);
                            }
                        }
                        for i in n_up_tiles.clone() {
                            if !rule.up_tiles.contains(&i){
                                rule.up_tiles.push(i);
                            }
                        }
                        for i in n_right_tiles.clone() {
                            if !rule.right_tiles.contains(&i){
                                rule.right_tiles.push(i);
                            }
                        }
                    }
                }  else {
                    self.adjacency_rules.insert(tile_name, n_rule);
                }
                n_up_tiles.clear();
                n_left_tiles.clear();
                n_down_tiles.clear();
                n_right_tiles.clear();

                // Rotate270
                let new_t = symmetry_dictionary[&rules.symmetry].eq[3];
                let new_s = &symmetry_dictionary[&rules.symmetry].sym[3];
                let tile_name = Self::new_tile_name(tile,new_t,&new_s);
                for tile in &rules.up_tiles {
                    let nnew_s = &self.adjacency_rules[tile].symmetry;
                    let nnew_t = symmetry_dictionary[nnew_s].eq[3];
                    if *nnew_s!="@".to_string(){
                        let ntile_name = Self::new_tile_name(tile,nnew_t,nnew_s);
                        n_left_tiles.push(ntile_name);
                    }
                }
                for tile in &rules.right_tiles {
                    let nnew_s = &self.adjacency_rules[tile].symmetry;
                    let nnew_t = symmetry_dictionary[nnew_s].eq[3];
                    if *nnew_s!="@".to_string(){
                        let ntile_name = Self::new_tile_name(tile,nnew_t,nnew_s);
                        n_up_tiles.push(ntile_name);
                    }
                }
                for tile in &rules.down_tiles {
                    let nnew_s = &self.adjacency_rules[tile].symmetry;
                    let nnew_t = symmetry_dictionary[nnew_s].eq[3];
                    if *nnew_s!="@".to_string(){
                        let ntile_name = Self::new_tile_name(tile,nnew_t,nnew_s);
                        n_right_tiles.push(ntile_name);
                    }
                }
                for tile in &rules.left_tiles {
                    let nnew_s = &self.adjacency_rules[tile].symmetry;
                    let nnew_t = symmetry_dictionary[nnew_s].eq[3];
                    if *nnew_s!="@".to_string(){
                        let ntile_name = Self::new_tile_name(tile,nnew_t,nnew_s);
                        n_down_tiles.push(ntile_name);
                    }
                }
                let n_rule = AdjacencyRule::new(new_s.clone(),n_up_tiles.clone(),n_right_tiles.clone(),n_down_tiles.clone(),n_left_tiles.clone());
                //n_adjancecy_rules.insert(tile_name, n_rule);
                if self.adjacency_rules.contains_key(&tile_name){
                    if let Some(rule) = self.adjacency_rules.get_mut(&tile_name){
                        for i in n_down_tiles.clone() {
                            if !rule.down_tiles.contains(&i){
                                rule.down_tiles.push(i);
                            }
                        }
                        for i in n_left_tiles.clone() {
                            if !rule.left_tiles.contains(&i){
                                rule.left_tiles.push(i);
                            }
                        }
                        for i in n_up_tiles.clone() {
                            if !rule.up_tiles.contains(&i){
                                rule.up_tiles.push(i);
                            }
                        }
                        for i in n_right_tiles.clone() {
                            if !rule.right_tiles.contains(&i){
                                rule.right_tiles.push(i);
                            }
                        }
                    }
                }  else {
                    self.adjacency_rules.insert(tile_name, n_rule);
                }
                n_up_tiles.clear();
                n_left_tiles.clear();
                n_down_tiles.clear();
                n_right_tiles.clear();

                // FlipH
                let new_t = symmetry_dictionary[&rules.symmetry].eq[4];
                let new_s = &symmetry_dictionary[&rules.symmetry].sym[4];
                let tile_name = Self::new_tile_name(tile,new_t,&new_s);
                for tile in &rules.up_tiles {
                    let nnew_s = &self.adjacency_rules[tile].symmetry;
                    let nnew_t = symmetry_dictionary[nnew_s].eq[4];
                    if *nnew_s!="@".to_string(){
                        let ntile_name = Self::new_tile_name(tile,nnew_t,nnew_s);
                        n_up_tiles.push(ntile_name);
                    }
                }
                for tile in &rules.right_tiles {
                    let nnew_s = &self.adjacency_rules[tile].symmetry;
                    let nnew_t = symmetry_dictionary[nnew_s].eq[4];
                    if *nnew_s!="@".to_string(){
                        let ntile_name = Self::new_tile_name(tile,nnew_t,nnew_s);
                        n_left_tiles.push(ntile_name);
                    }
                }
                for tile in &rules.down_tiles {
                    let nnew_s = &self.adjacency_rules[tile].symmetry;
                    let nnew_t = symmetry_dictionary[nnew_s].eq[4];
                    if *nnew_s!="@".to_string(){
                        let ntile_name = Self::new_tile_name(tile,nnew_t,nnew_s);
                        n_down_tiles.push(ntile_name);
                    }
                }
                for tile in &rules.left_tiles {
                    let nnew_s = &self.adjacency_rules[tile].symmetry;
                    let nnew_t = symmetry_dictionary[nnew_s].eq[4];
                    if *nnew_s!="@".to_string(){
                        let ntile_name = Self::new_tile_name(tile,nnew_t,nnew_s);
                        n_right_tiles.push(ntile_name);
                    }
                }
                let n_rule = AdjacencyRule::new(new_s.clone(),n_up_tiles.clone(),n_right_tiles.clone(),n_down_tiles.clone(),n_left_tiles.clone());
                //n_adjancecy_rules.insert(tile_name, n_rule);
                if self.adjacency_rules.contains_key(&tile_name){
                    if let Some(rule) = self.adjacency_rules.get_mut(&tile_name){
                        for i in n_down_tiles.clone() {
                            if !rule.down_tiles.contains(&i){
                                rule.down_tiles.push(i);
                            }
                        }
                        for i in n_left_tiles.clone() {
                            if !rule.left_tiles.contains(&i){
                                rule.left_tiles.push(i);
                            }
                        }
                        for i in n_up_tiles.clone() {
                            if !rule.up_tiles.contains(&i){
                                rule.up_tiles.push(i);
                            }
                        }
                        for i in n_right_tiles.clone() {
                            if !rule.right_tiles.contains(&i){
                                rule.right_tiles.push(i);
                            }
                        }
                    }
                }  else {
                    self.adjacency_rules.insert(tile_name, n_rule);
                }
                n_up_tiles.clear();
                n_left_tiles.clear();
                n_down_tiles.clear();
                n_right_tiles.clear();

                // FRotate90
                let new_t = symmetry_dictionary[&rules.symmetry].eq[5];
                let new_s = &symmetry_dictionary[&rules.symmetry].sym[5];
                let tile_name = Self::new_tile_name(tile,new_t,&new_s);
                for tile in &rules.up_tiles {
                    let nnew_s = &self.adjacency_rules[tile].symmetry;
                    let nnew_t = symmetry_dictionary[nnew_s].eq[5];
                    if *nnew_s!="@".to_string(){
                        let ntile_name = Self::new_tile_name(tile,nnew_t,nnew_s);
                        n_right_tiles.push(ntile_name);
                    }
                }
                for tile in &rules.right_tiles {
                    let nnew_s = &self.adjacency_rules[tile].symmetry;
                    let nnew_t = symmetry_dictionary[nnew_s].eq[5];
                    if *nnew_s!="@".to_string(){
                        let ntile_name = Self::new_tile_name(tile,nnew_t,nnew_s);
                        n_up_tiles.push(ntile_name);
                    }
                }
                for tile in &rules.down_tiles {
                    let nnew_s = &self.adjacency_rules[tile].symmetry;
                    let nnew_t = symmetry_dictionary[nnew_s].eq[5];
                    if *nnew_s!="@".to_string(){
                        let ntile_name = Self::new_tile_name(tile,nnew_t,nnew_s);
                        n_left_tiles.push(ntile_name);
                    }
                }
                for tile in &rules.left_tiles {
                    let nnew_s = &self.adjacency_rules[tile].symmetry;
                    let nnew_t = symmetry_dictionary[nnew_s].eq[5];
                    if *nnew_s!="@".to_string(){
                        let ntile_name = Self::new_tile_name(tile,nnew_t,nnew_s);
                        n_down_tiles.push(ntile_name);
                    }
                }
                let n_rule = AdjacencyRule::new(new_s.clone(),n_up_tiles.clone(),n_right_tiles.clone(),n_down_tiles.clone(),n_left_tiles.clone());
                //n_adjancecy_rules.insert(tile_name, n_rule);
                if self.adjacency_rules.contains_key(&tile_name){
                    if let Some(rule) = self.adjacency_rules.get_mut(&tile_name){
                        for i in n_down_tiles.clone() {
                            if !rule.down_tiles.contains(&i){
                                rule.down_tiles.push(i);
                            }
                        }
                        for i in n_left_tiles.clone() {
                            if !rule.left_tiles.contains(&i){
                                rule.left_tiles.push(i);
                            }
                        }
                        for i in n_up_tiles.clone() {
                            if !rule.up_tiles.contains(&i){
                                rule.up_tiles.push(i);
                            }
                        }
                        for i in n_right_tiles.clone() {
                            if !rule.right_tiles.contains(&i){
                                rule.right_tiles.push(i);
                            }
                        }
                    }
                }  else {
                    self.adjacency_rules.insert(tile_name, n_rule);
                }
                n_up_tiles.clear();
                n_left_tiles.clear();
                n_down_tiles.clear();
                n_right_tiles.clear();

                // FRotate180
                let new_t = symmetry_dictionary[&rules.symmetry].eq[6];
                let new_s = &symmetry_dictionary[&rules.symmetry].sym[6];
                let tile_name = Self::new_tile_name(tile,new_t,&new_s);
                for tile in &rules.up_tiles {
                    let nnew_s = &self.adjacency_rules[tile].symmetry;
                    let nnew_t = symmetry_dictionary[nnew_s].eq[6];
                    if *nnew_s!="@".to_string(){
                        let ntile_name = Self::new_tile_name(tile,nnew_t,nnew_s);
                        n_down_tiles.push(ntile_name);
                    }
                }
                for tile in &rules.right_tiles {
                    let nnew_s = &self.adjacency_rules[tile].symmetry;
                    let nnew_t = symmetry_dictionary[nnew_s].eq[6];
                    if *nnew_s!="@".to_string(){
                        let ntile_name = Self::new_tile_name(tile,nnew_t,nnew_s);
                        n_right_tiles.push(ntile_name);
                    }
                }
                for tile in &rules.down_tiles {
                    let nnew_s = &self.adjacency_rules[tile].symmetry;
                    let nnew_t = symmetry_dictionary[nnew_s].eq[6];
                    if *nnew_s!="@".to_string(){
                        let ntile_name = Self::new_tile_name(tile,nnew_t,nnew_s);
                        n_up_tiles.push(ntile_name);
                    }
                }
                for tile in &rules.left_tiles {
                    let nnew_s = &self.adjacency_rules[tile].symmetry;
                    let nnew_t = symmetry_dictionary[nnew_s].eq[6];
                    if *nnew_s!="@".to_string(){
                        let ntile_name = Self::new_tile_name(tile,nnew_t,nnew_s);
                        n_left_tiles.push(ntile_name);
                    }
                }
                let n_rule = AdjacencyRule::new(new_s.clone(),n_up_tiles.clone(),n_right_tiles.clone(),n_down_tiles.clone(),n_left_tiles.clone());
                //n_adjancecy_rules.insert(tile_name, n_rule);
                if self.adjacency_rules.contains_key(&tile_name){
                    if let Some(rule) = self.adjacency_rules.get_mut(&tile_name){
                        for i in n_down_tiles.clone() {
                            if !rule.down_tiles.contains(&i){
                                rule.down_tiles.push(i);
                            }
                        }
                        for i in n_left_tiles.clone() {
                            if !rule.left_tiles.contains(&i){
                                rule.left_tiles.push(i);
                            }
                        }
                        for i in n_up_tiles.clone() {
                            if !rule.up_tiles.contains(&i){
                                rule.up_tiles.push(i);
                            }
                        }
                        for i in n_right_tiles.clone() {
                            if !rule.right_tiles.contains(&i){
                                rule.right_tiles.push(i);
                            }
                        }
                    }
                }  else {
                    self.adjacency_rules.insert(tile_name, n_rule);
                }
                n_up_tiles.clear();
                n_left_tiles.clear();
                n_down_tiles.clear();
                n_right_tiles.clear();

                // FRotate270
                let new_t = symmetry_dictionary[&rules.symmetry].eq[7];
                let new_s = &symmetry_dictionary[&rules.symmetry].sym[7];
                let tile_name = Self::new_tile_name(tile,new_t,&new_s);
                for tile in &rules.up_tiles {
                    let nnew_s = &self.adjacency_rules[tile].symmetry;
                    let nnew_t = symmetry_dictionary[nnew_s].eq[7];
                    if *nnew_s!="@".to_string(){
                        let ntile_name = Self::new_tile_name(tile,nnew_t,nnew_s);
                        n_left_tiles.push(ntile_name);
                    }
                }
                for tile in &rules.right_tiles {
                    let nnew_s = &self.adjacency_rules[tile].symmetry;
                    let nnew_t = symmetry_dictionary[nnew_s].eq[7];
                    if *nnew_s!="@".to_string(){
                        let ntile_name = Self::new_tile_name(tile,nnew_t,nnew_s);
                        n_down_tiles.push(ntile_name);
                    }
                }
                for tile in &rules.down_tiles {
                    let nnew_s = &self.adjacency_rules[tile].symmetry;
                    let nnew_t = symmetry_dictionary[nnew_s].eq[7];
                    if *nnew_s!="@".to_string(){
                        let ntile_name = Self::new_tile_name(tile,nnew_t,nnew_s);
                        n_right_tiles.push(ntile_name);
                    }
                }
                for tile in &rules.left_tiles {
                    let nnew_s = &self.adjacency_rules[tile].symmetry;
                    let nnew_t = symmetry_dictionary[nnew_s].eq[7];
                    if *nnew_s!="@".to_string(){
                        let ntile_name = Self::new_tile_name(tile,nnew_t,nnew_s);
                        n_up_tiles.push(ntile_name);
                    }
                }
                let n_rule = AdjacencyRule::new(new_s.clone(),n_up_tiles.clone(),n_right_tiles.clone(),n_down_tiles.clone(),n_left_tiles.clone());
                //n_adjancecy_rules.insert(tile_name, n_rule);
                if self.adjacency_rules.contains_key(&tile_name){
                    if let Some(rule) = self.adjacency_rules.get_mut(&tile_name){
                        for i in n_down_tiles.clone() {
                            if !rule.down_tiles.contains(&i){
                                rule.down_tiles.push(i);
                            }
                        }
                        for i in n_left_tiles.clone() {
                            if !rule.left_tiles.contains(&i){
                                rule.left_tiles.push(i);
                            }
                        }
                        for i in n_up_tiles.clone() {
                            if !rule.up_tiles.contains(&i){
                                rule.up_tiles.push(i);
                            }
                        }
                        for i in n_right_tiles.clone() {
                            if !rule.right_tiles.contains(&i){
                                rule.right_tiles.push(i);
                            }
                        }
                    }
                }  else {
                    self.adjacency_rules.insert(tile_name, n_rule);
                }
                n_up_tiles.clear();
                n_left_tiles.clear();
                n_down_tiles.clear();
                n_right_tiles.clear();
            }
        }
        return self.clone();
    }

    // Given a tile path and a number creates a new tile name (ex: tile.png with number 3 gives tile-3.png)
    pub fn nnew_tile_name(old_tile: &String,n: u32) -> String {
        if n != 0 {
            if let Some(dot_index) = old_tile.rfind('.'){
                let new_tile = format!("{}-{}{}", &old_tile[..dot_index],n.to_string(),&old_tile[dot_index..]);
                return new_tile;
            } else {
                return "Null".to_string();
            }
        } else {
            return old_tile.to_string();
        }
    }

    // Given a tile path its symmetry and a transformation n, creates a new tile name
    pub fn new_tile_name(old_tile: &String,n: u32, symmetry: &str) -> String {
        if n!=0 {
            match symmetry {
                a if (a=="B" || a=="T" || a=="L" || a=="Q") => {
                    // if tile was already transformed calculate new n
                    if let Some(n_index) = old_tile.rfind('-'){
                        let old_n = old_tile.chars().nth(n_index+1).expect("Error char not found").to_digit(10).expect("Can't convert to number");
                        let new_n = (old_n+n)%4;
                        let mut new_c = String::new();
                        if new_n != 0 {
                            new_c = format!("-{}",new_n.to_string());    
                        }
                        let new_tile = format!("{}{}{}", &old_tile[..n_index],new_c,&old_tile[n_index+2..]);
                        return new_tile;
                    } else {
                        if let Some(dot_index) = old_tile.rfind('.'){
                            let new_tile = format!("{}-{}{}", &old_tile[..dot_index],n.to_string(),&old_tile[dot_index..]);
                            return new_tile;
                        } else {
                            return "Null".to_string();
                        }
                    }
                },
                a if (a=="I" || a=="/") => {
                    // if tile was already transformed calculate new n
                    if let Some(n_index) = old_tile.rfind('-'){
                        let old_n = old_tile.chars().nth(n_index+1).expect("Error char not found").to_digit(10).expect("Can't convert to number");
                        let new_n = (old_n+n)%2;
                        let mut new_c = String::new();
                        if new_n != 0 {
                            new_c = format!("-{}",new_n.to_string());    
                        }
                        let new_tile = format!("{}-{}{}", &old_tile[..n_index],new_c.to_string(),&old_tile[n_index+2..]);
                        return new_tile;
                    } else {
                        if let Some(dot_index) = old_tile.rfind('.'){
                            let new_tile = format!("{}-{}{}", &old_tile[..dot_index],n.to_string(),&old_tile[dot_index..]);
                            return new_tile;
                        } else {
                            return "Null".to_string();
                        }
                    }
                },
                _ => {
                    if let Some(dot_index) = old_tile.rfind('.'){
                        let new_tile = format!("{}-{}{}", &old_tile[..dot_index],n.to_string(),&old_tile[dot_index..]);
                        return new_tile;
                    } else {
                        return "Null".to_string();
                    }
                }
            }
        } else {
            return old_tile.to_string();
        }
    }
}