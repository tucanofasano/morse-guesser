use std::collections::HashMap;

pub fn morse_map() -> HashMap<char, &'static str> {
    let mut map = HashMap::new();

    map.insert('A', ".-");
    map.insert('B', "-...");
    map.insert('C', "-.-.");
    map.insert('D', "-..");
    map.insert('E', ".");
    map.insert('F', "..-.");
    map.insert('G', "--.");
    map.insert('H', "....");
    map.insert('I', "..");
    map.insert('J', ".---");
    map.insert('K', "-.-");
    map.insert('L', ".-..");
    map.insert('M', "--");
    map.insert('N', "-.");
    map.insert('O', "---");
    map.insert('P', ".--.");
    map.insert('Q', "--.-");
    map.insert('R', ".-.");
    map.insert('S', "...");
    map.insert('T', "-");
    map.insert('U', "..-");
    map.insert('V', "...-");
    map.insert('W', ".--");
    map.insert('X', "-..-");
    map.insert('Y', "-.--");
    map.insert('Z', "--..");

    map
}