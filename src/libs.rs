/* pub fn remove_accent(s: &str) -> String {
    let mut result: String = String::with_capacity(s.len());

    for c in s.chars() {
        match c {
            'á' | 'à' | 'ã' | 'â' => result.push('a'),
            'é' | 'è' | 'ẽ' | 'ê' => result.push('e'),
            'í' | 'ì' | 'ĩ' | 'î' => result.push('i'),
            'ó' | 'ò' | 'õ' | 'ô' => result.push('o'),
            'ú' | 'ù' | 'ũ' | 'û' => result.push('u'),
            'ç' => result.push('c'),
            _ => result.push(c),
        }
    }

    result
} */
