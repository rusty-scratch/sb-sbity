use super::*;

/// Thanks griffpatch, well he didn't give me permission but anyways he's awesome. (https://scratch.mit.edu/users/griffpatch/)
#[test]
fn slitcherio_project() {}

/// Thanks flowing_code. No permission either; they're paid with exposure. (https://scratch.mit.edu/users/flowing_code/)
/// ⣿⣿⣿⣿⣿⣿⣿⣿⣿⠿⠛⠋⣉⣉⣉⣉⣉⣉⠙⠛⠿⣿⣿⣿⣿⣿⣿⣿⣿⣿
/// ⣿⣿⣿⣿⣿⡿⠟⢁⣤⣶⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣶⣤⡈⠻⢿⣿⣿⣿⣿⣿
/// ⣿⣿⣿⡿⠋⣠⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⣄⠙⢿⣿⣿⣿
/// ⣿⣿⡟⢀⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⡀⢻⣿⣿
/// ⣿⡟⢠⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡄⢻⣿
/// ⣿⢀⣿⣿⣿⠟⠁⣠⣴⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣦⣄⠈⠻⣿⣿⣿⡀⣿
/// ⡇⢸⣿⣿⠋⣠⡾⠿⠛⠛⠛⠿⣿⣿⣿⣿⣿⣿⠿⠛⠛⠛⠻⢷⣄⠙⣿⣿⡇⢸
/// ⡇⢸⣿⣿⣾⣿⢀⣠⣤⣤⣤⣤⣀⣿⣿⣿⣿⣀⣤⣤⣤⣤⣄⡀⣿⣷⣾⣿⡇⢸
/// ⡇⠸⠟⣫⣥⣶⣧⠹⠿⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠿⠏⣼⣶⣬⣍⠻⠇⢸
/// ⡧⣰⣿⣿⣿⣿⣿⢰⣦⣤⣤⣤⣤⣤⣤⣤⣤⣤⣤⣤⣤⣴⡆⣿⣿⣿⣿⣿⣆⢼
/// ⡇⣿⣿⣿⣿⣿⡟⠈⠙⠛⠻⠿⠿⠿⠿⠿⠿⠿⠿⠟⠛⠋⠁⢻⣿⣿⣿⣿⣿⢸
/// ⣿⣌⡻⠿⠿⢋⣴⣦⡀⡀⡀⡀⡀⡀⡀⡀⡀⡀⡀⡀⡀⢀⣴⣦⡙⠿⠿⢟⣡⣾
/// ⣿⣿⣿⣷⣄⠙⢿⣿⣿⣶⣤⣀⡀⡀⡀⡀⡀⡀⣀⣤⣶⣿⣿⡿⠋⣠⣾⣿⣿⣿
/// ⣿⣿⣿⣿⣿⣷⣦⣉⠛⠿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠿⠛⣉⣴⣾⣿⣿⣿⣿⣿
/// ⣿⣿⣿⣿⣿⣿⣿⣿⣿⣶⣤⣌⣉⣉⣉⣉⣉⣉⣡⣤⣶⣿⣿⣿⣿⣿⣿⣿⣿⣿
#[test]
fn orbit_project() {}

/// now i made this one
#[test]
fn simple_project() {
    let project: &str = include_str!("test_case\\simple_project.json");
    // let project: Json = serde_json::from_str(project).unwrap();
    let project: Project = serde_json::from_str(project).unwrap();
    dbg!(project);
}
