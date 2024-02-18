mod sorted_set; // sorted_set 모듈 선언

use sorted_set::SortedSet; // SortedSet 구조체를 현재 범위로 가져옴


fn main() {
    let mut sorted_set = SortedSet::new();
    sorted_set.z_add("a".to_string(), 1);
    sorted_set.z_add("b".to_string(), 2);
    sorted_set.z_add("c".to_string(), 3);

    println!("{:?}", &sorted_set);
}