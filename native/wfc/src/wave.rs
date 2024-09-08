use core::f32;
use std::collections::HashMap;
use std::collections::BTreeMap;
use std::f32::consts::E;
use crate::tile_set::TileSet;
use crate::tile_set::RuleSet;
use crate::tile_set::AdjacencyRule;
use rand::Rng;
use rand::seq::SliceRandom;

#[derive(Debug,Clone)]
pub struct Wave<'a> {
    pub size: (i32, i32), // Size of the wave
    pub regions: BTreeMap<(i32, i32), Region<'a>> //Map that for every position of the wave gives a region (like a matrix)
}

impl Wave<'_> {
    // Creates a new wave given a tile set, size of wave and list of probabilities associated with each tile
    pub fn new<'a>(tile_set: &'a TileSet, size: (i32, i32), probabilities: HashMap<String, u32>, rule_set: &'a RuleSet) -> Wave<'a>  {
        let (x,y) = size;
        let mut region = Region::new(tile_set,probabilities);
        region.update_entropy(&rule_set);
        let mut regions = BTreeMap::new();
        for i in 0..y {
            for j in 0..x {
                regions.insert((j,i), region.clone());
            }
        }
        Wave { size, regions }
    }

    // List all of the tiles in a fully collapsed wave
    pub fn list_tiles(&self) -> String {
        let mut result = "".to_string();
        let (x,y) = self.size;
        for i in 0..y {
            for j in 0..x {
                let region = self.regions.get(&(j,i)).unwrap();
                let tile = region.get_tile();
                result.push_str(&tile);
                result.push_str(" ");
            }
        }
        return result;
    }

    // Collapses a wave
    pub fn collapse(&mut self, tile_set_size: u32, rule_set: &RuleSet) -> (i32, i32) {
        // Get the lowest entropy region if it doesn't exist than there is no more regions to collapse
        let (x,y) = self.lowest_entropy(tile_set_size);
        if x>=0 && y>=0 {
            let region: &mut Region<'_> = self.regions.get_mut(&(x,y)).unwrap();
            region.collapse_with_reach(rule_set, (x,y),self.size);
            //region.collapse();
            //dbg!(self);
        }
        return (x,y);
    }

    // Forces a region to collapse into a specific tile
    pub fn set_pos(&mut self,(x,y): (i32,i32),tile: String, rule_set: &RuleSet) {
        if self.regions.contains_key(&(x,y)) {
            let region: &mut Region<'_> = self.regions.get_mut(&(x,y)).unwrap();
            region.set_tile(tile);
            self.start_propagate((x,y), rule_set);
        } else {
            dbg!("Invalid Position");
        }
    }

    // Collapses a wave until there is no more regions to collapse
    pub fn loop_propagate(&mut self, tile_set_size: u32, rule_set: &RuleSet, _results_path: Option<&String>) -> &str {
        /*match results_path {
            Some(path) => render::render_wave(&self,format!("{}it0.png",path),&rule_set.tiles_path),
            None => ()
        }*/
        // Collapse a region
        let (mut x, mut y) = self.collapse(tile_set_size, rule_set);
        //let mut it = 1;
        while x>=0 && y>=0 {
            /*match results_path {
                Some(path) => render::render_wave(&self,format!("{}it{}.png",path,it),&rule_set.tiles_path),
                None => ()
            }*/

            //render::render_wave(&self,format!("results3/it{}.png",it));
            // Propagate effects of region collapse
            self.start_propagate((x,y),rule_set);
            //dbg!(self.clone());
            (x,y) = self.collapse(tile_set_size,rule_set);
            //it+=1;
        }
        if (x,y) == (-1,-1){
            return "ok_wave";
        } else {
            return "invalid_wave";
        }
    }
    // Gets the lowest entropy region (gets a random region from the regions with the least amount of possible tile)
    pub fn lowest_entropy(&self, tile_set_size: u32) -> (i32, i32) {
        let mut lowest = f32::INFINITY;
        //let mut lowest = tile_set_size+1;
        let mut all_pos = Vec::new();
        for (k, v) in &self.regions {
            let temp = v.entropy;
            if temp.is_nan() {
                return (-2,-2);
            } else if temp < lowest && temp > 0.0 {
                lowest = temp;
                all_pos.clear();
                all_pos.push(k);
            } else if temp == lowest {
                all_pos.push(k);
            }
        }

        let pos;

        if all_pos.len() == 0 {
            pos = (-1,-1);
        } else {
            let mut rng = rand::thread_rng();
            pos = **all_pos.choose(&mut rng).unwrap();
        }
        return pos;
    }

    // Propagate the effect of collapsing a region
    pub fn start_propagate(&mut self, i_pos: (i32,i32), rule_set: &RuleSet) {
        let mut pos_v = Vec::new();
        let (fx,fy) = self.size;
        pos_v.push(i_pos);
        while pos_v.len()>0 {
            let mut new_pos_v = Vec::new();
            // Checks all of the adjacent positions to the position being propagated and update them with if they are a legal position than update them and if the update changed them add them to the list of positions being propagated
            for (x,y) in &pos_v {
                let vecs = self.regions.get(&(*x,*y)).unwrap().get_dir_vectors(rule_set);
                if *x>0 {
                    let region: &mut Region<'_> = self.regions.get_mut(&(x-1,*y)).unwrap();
                    if region.update(vecs[0].clone(),rule_set) { new_pos_v.push((x-1,*y)); }
                }   
                if *y>0 {
                    let region: &mut Region<'_> = self.regions.get_mut(&(*x,y-1)).unwrap();
                    if region.update(vecs[1].clone(),rule_set) { new_pos_v.push((*x,y-1)); }
                }   
                if *x<fx-1 {
                    let region: &mut Region<'_> = self.regions.get_mut(&(x+1,*y)).unwrap();
                    if region.update(vecs[2].clone(),rule_set) { new_pos_v.push((x+1,*y)); }
                }   
                if *y<fy-1 {
                    let region: &mut Region<'_> = self.regions.get_mut(&(*x,y+1)).unwrap();
                    if region.update(vecs[3].clone(),rule_set) { new_pos_v.push((*x,y+1)); }
                }   
            }
            pos_v.clear();
            pos_v=new_pos_v.clone();
        }
    }
}

#[derive(Debug,Clone)]
pub struct Region<'a> {
    pub superposition: HashMap<&'a String, u32>, //Map of possible tiles the region can be in and their probabilities
    pub entropy: f32 //Number of tiles in the the region can be
}

impl Region<'_> {
    // Creates a new region
    pub fn new(tile_set: &TileSet, probabilities: HashMap<String, u32>) -> Region {
        let mut superposition = HashMap::new();
        let entropy = tile_set.size;
        for i in 0..entropy {
            let k = &tile_set.tiles[i as usize];
            if probabilities.contains_key(k){
                superposition.insert(k, *probabilities.get(k).unwrap());
            } else {
                superposition.insert(k, 100);   
            }
        }
        Region {
            superposition,
            entropy: entropy as f32
        }
    }

    pub fn update_entropy(&mut self, rule_set: &RuleSet){
        let tiles: Vec<_> = self.superposition.keys().map(|v| v.clone()).collect();
        let weights = rule_set.get_weights(tiles);
        let sum_weights: f32 = weights.iter().sum();
        let sum_log_weights: f32 = weights.iter().map(|v| v*v.log(E)).sum();
        let res = sum_weights.log(E) - (sum_log_weights / sum_weights);
        self.entropy = res;
    }

    // Gets the first tile in a region superposition (Suppose to be used when region was collapsed and there is only one tile in the superposition)
    pub fn get_tile(&self) -> String {
        return self.superposition.keys().next().unwrap().to_string();
    }

    // Forces a region to collapse into a specific tile
    pub fn set_tile(&mut self, tile: String) {
        let mut newsp = HashMap::new();
        let (k,v) = self.superposition.iter_mut().find(|(&k,_v)| *k == tile).unwrap();
        newsp.insert(*k,*v);
        self.superposition = newsp;
        self.entropy = 0.0;
    }

    // Collapses a region choosing a tile randomly given the probabilities of possible tile outcomes
    pub fn collapse(&mut self){
        //let mut rng = rand::thread_rng();
        let mut newsp = HashMap::new();
        let rk = Self::probability_func(self.clone().superposition); 
        //let (k,v) = self.superposition.iter_mut().find(|(&k,_v)| *k == rk).unwrap();
        let (k,v) = self.superposition.get_key_value(&rk).unwrap();
        newsp.insert(*k,*v);
        self.superposition = newsp;
        self.entropy = 0.0;
    }

    // Same as collapse but also check for tiles that have enough reach (decreases number of errors when ruleset is not complete)
    pub fn collapse_with_reach(&mut self, rule_set: &RuleSet, pos: (i32,i32), size: (i32,i32)){
        //let mut rng = rand::thread_rng();
        let mut newsp = HashMap::new();

        let rk = Self::probability_func(self.clone().filter_for_reach(rule_set, pos, size)); 
        let (k,v) = self.superposition.get_key_value(&rk).unwrap();
        //let (k,v) = filtered_superposition.iter_mut().find(|(&k,_v)| *k == rk).unwrap();
        newsp.insert(*k,*v);
        self.superposition = newsp;
        self.entropy = 0.0;
    }

    // Filters for the tiles that have enough reach in the superposition, 
    pub fn filter_for_reach(&self, rule_set: &RuleSet, (xp,yp): (i32, i32), (xs,ys): (i32, i32)) -> HashMap<&String, u32>{
        //amount of spaces between the position of the region and the edges of the map
        let needed_reach = vec![yp,xp,ys-yp-1,xs-xp-1];
        let res: HashMap<_, _> = self.superposition.iter().filter(|(&k,_v)| rule_set.can_reach(k.to_string(), needed_reach.clone())).map(|(k,v)| (*k,*v)).collect();
        return res;
    }

    // Given all of the possible tile outcomes and their probabilities create a probability function and returns a random tile in that function
    // Ex: (If tileA has double the normal probability than the probability function will look like ((0,100) => tileA, (100,300) => tileB, (300,400) => tileC)) than a random number between 0 and 400 is picked and the corresponding tile is picked 
    pub fn probability_func(superposition: HashMap<&String, u32>) -> String{
        let mut min;
        let mut max = 0;
        let mut prob_func: HashMap<(u32, u32), &String> = HashMap::new();
        for (k,v) in superposition {
            if v>0 {
                min = max;
                max = max+v;
                prob_func.insert((min,max), k);
            }
        }
        
        let mut rng = rand::thread_rng();
        let n = rng.gen_range(0..max);
        let mut rk = "Null".to_string();
        for ((min,max), k) in prob_func {
            if n>=min && n<max {
                rk = (*k.clone()).to_string();
                break;
            }
        }
        return rk;
    }
    
    // Gets the possible tiles in the regions adjacent to this region
    pub fn get_dir_vectors(&self, rule_set: &RuleSet) -> Vec<Vec<String>>{
        let mut left = Vec::new();
        let mut up = Vec::new();
        let mut right = Vec::new();
        let mut down = Vec::new();
        let mut res = Vec::new();
        for (k,_v) in &self.superposition {
            let adj_rule : &AdjacencyRule = rule_set.adjacency_rules.get(&k as &str).unwrap();
            left.append(&mut adj_rule.left_tiles.clone().into_iter().filter(|k| !left.contains(k)).collect());
            up.append(&mut adj_rule.up_tiles.clone().into_iter().filter(|k| !up.contains(k)).collect()); 
            right.append(&mut adj_rule.right_tiles.clone().into_iter().filter(|k| !right.contains(k)).collect());
            down.append(&mut adj_rule.down_tiles.clone().into_iter().filter(|k| !down.contains(k)).collect());
        }
        res.push(left);
        res.push(up);
        res.push(right);
        res.push(down);

        return res;
    }

    // Updates a region with new possible tile outcomes, returns true if updates changed region superposition
    pub fn update(&mut self, updates: Vec<String>, rule_set: &RuleSet) -> bool {
        let old_len = self.superposition.iter().count() as f32;
        self.superposition = self.superposition.iter().filter(|(k,_v)| updates.contains(k)).map(|(k,v)| (*k,*v)).collect();
        let new_len = self.superposition.len() as u32;
        let res = (new_len as f32) < old_len;
        //self.entropy = new_len as f32;
        if res { self.update_entropy(rule_set); }
        return res;
    }
}