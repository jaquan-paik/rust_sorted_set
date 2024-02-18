use std::collections::btree_map::Range;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::ops::RangeBounds;


#[derive(Debug)]
pub struct SortedSet{
    score_index : BTreeMap<u32, HashSet<String>>,
    member_hash_map : HashMap<String,u32>,
}

impl SortedSet{
    pub fn new() -> SortedSet{
        SortedSet{
            score_index : BTreeMap::new(),
            member_hash_map : HashMap::new()
        }
    }
    pub fn z_add(&mut self, member : String, score : u32) {
        if let Some(old_score) = self.member_hash_map.insert(member.clone(),score){
            self.remove_score_index(&old_score, &member);
        }
        self.score_index.entry(score).or_insert_with(HashSet::new).insert(member);
    }

    pub fn z_rem(&mut self, member : &String) {
        if let Some(score) = self.member_hash_map.get(member).cloned(){
            self.remove_score_index(&score, &member);
        }
        self.member_hash_map.remove(member);
    }


    pub fn z_range_by_score<R>(&self, range: R) -> Range<u32,HashSet<String>>
    where
        R: RangeBounds<u32>
    {
         self.score_index.range(range)
    }

    fn remove_score_index(&mut self, score : &u32, member : &String){
        if let Some(hash_set) = self.score_index.get_mut(&score){
            hash_set.remove(member);
            if hash_set.is_empty(){
                self.score_index.remove(&score);
            }
        }
    }
}
