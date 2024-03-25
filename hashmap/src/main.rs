use std::collections::HashMap;


//example 1
// fn main() {
//     let mut scores:HashMap<&str, i32> = HashMap::new();
//     scores.insert("Sunface", 98);
//     scores.insert("Daniel", 88);
//     scores.insert("Ash", 79);
//     scores.insert("Sunflower", 108);

//     //get returns an Option<&V>
//     let score: Option<&i32> = scores.get("Sunface");
//     assert_eq!(score, Some(&98));

//     if scores.contains_key("Daniel") {
//         let score:i32 = scores["Daniel"];
//         assert_eq!(score, 88);

//         scores.remove("Daniel");
//     }

//     assert_eq!(scores.len(), 3);
//     for (name, score) in scores {
//         println!("The score of {} is {}", name, score);
//     }

// }


// //example 2
// fn main() {
//     let teams: [(&str, i32); 3] = [
//         ("Chinese team", 100),
//         ("American team", 10),
//         ("France team", 50),
//     ];

//     let mut teams_map1: HashMap<&str, i32> = HashMap::new();
//     for team in &teams {
//         teams_map1.insert(team.0, team.1);
//     }
//     let teams_map2: HashMap<&str, i32> = HashMap::from(teams);
//     // let teams_map2: HashMap<&str, i32> = teams.into_iter().collect();

//     assert_eq!(teams_map1, teams_map2);

//     println!("success");
// }

//example 3
fn main() {
    
}