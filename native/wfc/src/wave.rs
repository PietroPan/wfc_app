use std::collections::HashMap;
use std::collections::BTreeMap;
use crate::tile_set::TileSet;
use crate::tile_set::RuleSet;
use crate::tile_set::AdjacencyRule;
use rand::Rng;
use rand::seq::SliceRandom;
use crate::render;

#[derive(Debug)]
pub struct Wave<'a> {
    pub size: (i32, i32),
    pub regions: BTreeMap<(i32, i32), Region<'a>>
}

impl Wave<'_> {
    pub fn new(tile_set: &TileSet, size: (i32, i32), probabilities: HashMap<String, u32>) -> Wave {
        let (x,y) = size;
        let region = Region::new(tile_set,probabilities);
        let mut regions = BTreeMap::new();
        for i in 0..y {
            for j in 0..x {
                regions.insert((j,i), region.clone());
            }
        }
        Wave { size, regions }
    }

    pub fn list_tiles(&self) -> String {
        let mut result = "".to_string();
        let (x,y) = self.size;
        for i in 0..x {
            for j in 0..y {
                let region = self.regions.get(&(j,i)).unwrap();
                let tile = region.get_tile();
                result.push_str(&tile);
                result.push_str(" ");
            }
        }
        return result;
    }

    pub fn collapse(&mut self, tile_set_size: u32) -> (i32, i32) {
        let (x,y) = self.lowest_entropy(tile_set_size);
        if (x,y) != (-1,-1) {
            let region: &mut Region<'_> = self.regions.get_mut(&(x,y)).unwrap();
            region.collapse();
        }
        return (x,y);
    }

    pub fn set_pos(&mut self,(x,y): (i32,i32),tile: String, rule_set: &RuleSet) {
        if self.regions.contains_key(&(x,y)) {
            let region: &mut Region<'_> = self.regions.get_mut(&(x,y)).unwrap();
            region.set_tile(tile);
            self.start_propagate((x,y), rule_set);
        } else {
            dbg!("Invalid Position");
        }
    }

    pub fn loop_propagate(&mut self, tile_set_size: u32, rule_set: &RuleSet, results_path: Option<&String>) {
        /*match results_path {
            Some(path) => render::render_wave(&self,format!("{}it0.png",path),&rule_set.tiles_path),
            None => ()
        }*/
        let mut pos = self.collapse(tile_set_size);
        let mut it = 1;
        while pos!=(-1,-1) {
            /*match results_path {
                Some(path) => render::render_wave(&self,format!("{}it{}.png",path,it),&rule_set.tiles_path),
                None => ()
            }*/

            //render::render_wave(&self,format!("results3/it{}.png",it));
            self.start_propagate(pos,rule_set);
            pos = self.collapse(tile_set_size);
            it+=1;
        }
    }

    pub fn lowest_entropy(&self, tile_set_size: u32) -> (i32, i32) {
        let mut lowest = tile_set_size+1;
        let mut all_pos = Vec::new();
        for (k, v) in &self.regions {
            let temp = v.entropy;
            if temp < lowest && temp > 1 {
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

    pub fn start_propagate(&mut self, i_pos: (i32,i32), rule_set: &RuleSet) {
        let mut pos_v = Vec::new();
        let (fx,fy) = self.size;
        pos_v.push(i_pos);
        while pos_v.len()>0 {
            let mut new_pos_v = Vec::new();
            for (x,y) in &pos_v {
                let vecs = self.regions.get(&(*x,*y)).unwrap().get_dir_vectors(rule_set);
                if *x>0 {
                    let region: &mut Region<'_> = self.regions.get_mut(&(x-1,*y)).unwrap();
                    if region.update(vecs[0].clone()) { new_pos_v.push((x-1,*y)); }
                }   
                if *y>0 {
                    let region: &mut Region<'_> = self.regions.get_mut(&(*x,y-1)).unwrap();
                    if region.update(vecs[1].clone()) { new_pos_v.push((*x,y-1)); }
                }   
                if *x<fx-1 {
                    let region: &mut Region<'_> = self.regions.get_mut(&(x+1,*y)).unwrap();
                    if region.update(vecs[2].clone()) { new_pos_v.push((x+1,*y)); }
                }   
                if *y<fy-1 {
                    let region: &mut Region<'_> = self.regions.get_mut(&(*x,y+1)).unwrap();
                    if region.update(vecs[3].clone()) { new_pos_v.push((*x,y+1)); }
                }   
            }
            pos_v.clear();
            pos_v=new_pos_v.clone();
        }
    }
}

#[derive(Debug,Clone)]
pub struct Region<'a> {
    pub superposition: HashMap<&'a String, u32>,
    pub entropy: u32
}

impl Region<'_> {
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
            entropy
        }
    }

    pub fn get_tile(&self) -> String {
        return self.superposition.keys().next().unwrap().to_string();
    }

    pub fn set_tile(&mut self, tile: String) {
        let mut newsp = HashMap::new();
        let (k,v) = self.superposition.iter_mut().find(|(&k,_v)| *k == tile).unwrap();
        newsp.insert(*k,*v);
        self.superposition = newsp;
        self.entropy = 1;
    }

    pub fn collapse(&mut self){
        //let mut rng = rand::thread_rng();
        let mut newsp = HashMap::new();
        let rk = Self::probability_func(self.clone().superposition); 
        let (k,v) = self.superposition.iter_mut().find(|(&k,_v)| *k == rk).unwrap();
        newsp.insert(*k,*v);
        self.superposition = newsp;
        self.entropy = 1;
    }

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

    pub fn update(&mut self, updates: Vec<String>) -> bool {
        self.superposition = self.superposition.iter().filter(|(k,_v)| updates.contains(k)).map(|(k,v)| (*k,*v)).collect();
        let new_len = self.superposition.len() as u32;
        let res = new_len < self.entropy;
        self.entropy = new_len;
        return res;
    }
}