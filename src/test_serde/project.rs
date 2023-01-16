use super::*;

test_json! {
    Project {
        // Thanks griffpatch, well he didn't give me permission but anyways he's awesome. (https://scratch.mit.edu/users/griffpatch/)
        slitcherio_project => include_str!("test_case\\slitcherio.json"),

        // Thanks flowing_code. No permission either; they're paid with exposure. (https://scratch.mit.edu/users/flowing_code/)
        // ⣿⣿⣿⣿⣿⣿⣿⣿⣿⠿⠛⠋⣉⣉⣉⣉⣉⣉⠙⠛⠿⣿⣿⣿⣿⣿⣿⣿⣿⣿
        // ⣿⣿⣿⣿⣿⡿⠟⢁⣤⣶⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣶⣤⡈⠻⢿⣿⣿⣿⣿⣿
        // ⣿⣿⣿⡿⠋⣠⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⣄⠙⢿⣿⣿⣿
        // ⣿⣿⡟⢀⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⡀⢻⣿⣿
        // ⣿⡟⢠⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡄⢻⣿
        // ⣿⢀⣿⣿⣿⠟⠁⣠⣴⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣦⣄⠈⠻⣿⣿⣿⡀⣿
        // ⡇⢸⣿⣿⠋⣠⡾⠿⠛⠛⠛⠿⣿⣿⣿⣿⣿⣿⠿⠛⠛⠛⠻⢷⣄⠙⣿⣿⡇⢸
        // ⡇⢸⣿⣿⣾⣿⢀⣠⣤⣤⣤⣤⣀⣿⣿⣿⣿⣀⣤⣤⣤⣤⣄⡀⣿⣷⣾⣿⡇⢸
        // ⡇⠸⠟⣫⣥⣶⣧⠹⠿⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠿⠏⣼⣶⣬⣍⠻⠇⢸
        // ⡧⣰⣿⣿⣿⣿⣿⢰⣦⣤⣤⣤⣤⣤⣤⣤⣤⣤⣤⣤⣤⣴⡆⣿⣿⣿⣿⣿⣆⢼
        // ⡇⣿⣿⣿⣿⣿⡟⠈⠙⠛⠻⠿⠿⠿⠿⠿⠿⠿⠿⠟⠛⠋⠁⢻⣿⣿⣿⣿⣿⢸
        // ⣿⣌⡻⠿⠿⢋⣴⣦⡀⡀⡀⡀⡀⡀⡀⡀⡀⡀⡀⡀⡀⢀⣴⣦⡙⠿⠿⢟⣡⣾
        // ⣿⣿⣿⣷⣄⠙⢿⣿⣿⣶⣤⣀⡀⡀⡀⡀⡀⡀⣀⣤⣶⣿⣿⡿⠋⣠⣾⣿⣿⣿
        // ⣿⣿⣿⣿⣿⣷⣦⣉⠛⠿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠿⠛⣉⣴⣾⣿⣿⣿⣿⣿
        // ⣿⣿⣿⣿⣿⣿⣿⣿⣿⣶⣤⣌⣉⣉⣉⣉⣉⣉⣡⣤⣶⣿⣿⣿⣿⣿⣿⣿⣿⣿
        orbit_project => include_str!("test_case\\orbit_project.json"),

        // now i made this one. cool right?
        simple_project => include_str!("test_case\\simple_project.json")
    }
}
