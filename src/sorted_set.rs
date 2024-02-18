use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;

pub enum SortedSetNode{
    Node(String, u32),
    Nil
}
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

    pub fn stringify(&self ) -> String{
        // 쭉 score index 순으로 출력
        String::new()
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