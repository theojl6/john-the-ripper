// brute force passwords of length 4
pub fn brute_force() -> Vec<String> {
    let password_characters = vec![
        "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R",
        "S", "T", "U", "V", "W", "X", "Y", "Z", // Uppercase letters
        "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r",
        "s", "t", "u", "v", "w", "x", "y", "z", // Lowercase letters
        "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", // Digits
        "!", "@", "#", "$", "%", "^", "&", "*", "(", ")", "-", "_", "=",
        "+", // Special characters
        "{", "}", "[", "]", ":", ";", "\'", "\"", "<", ">", ",", ".", "?", "/",
        "\\", // Additional special characters
    ];
    let mut res = vec![];
    for i in 0..password_characters.len() {
        for j in 0..password_characters.len() {
            for k in 0..password_characters.len() {
                for l in 0..password_characters.len() {
                    res.push(
                        password_characters[i].to_owned()
                            + password_characters[j]
                            + password_characters[k]
                            + password_characters[l],
                    );
                }
            }
        }
    }
    res
}
