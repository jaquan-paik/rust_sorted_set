mod sorted_set;
use sorted_set::SortedSet;


fn main() {
    let mut sorted_set = SortedSet::new();
    sorted_set.z_add("a".to_string(), 1);
    sorted_set.z_add("b".to_string(), 2);
    sorted_set.z_add("c".to_string(), 3);
    sorted_set.z_add("c".to_string(), 1);
    sorted_set.z_add("d".to_string(), 4);
    sorted_set.z_add("e".to_string(), 1);
    sorted_set.z_rem(&"e".to_string());



    println!("{:?}", &sorted_set);
    println!("{:?}", &sorted_set.z_range_by_score(2..))
}