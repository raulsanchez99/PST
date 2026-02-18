fn next_token(s: &str, pos: &mut usize) -> Option<char> {
    let chars: Vec<char> = s.chars().collect();
    while *pos < chars.len() && chars[*pos].is_whitespace() {
        *pos += 1;
    }
    if *pos >= chars.len() {
        return None;
    }
    let c = chars[*pos];
    *pos += 1;
    Some(c)
}

#[derive(Debug)]
struct Pila<T> {
    data: Vec<T>,
}

impl<T> Pila<T> {
    fn new() -> Self {
        Pila { data: Vec::new() }
    }

    fn push(&mut self, val: T) {
        self.data.push(val);
    }

    fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    fn vacia(&self) -> bool {
        self.data.is_empty()
    }
}

fn parentesis_equilibrados(s: &str) -> bool {
    let mut pila_paren = Pila::new();
    let mut pila_llaves = Pila::new();
    let mut pos = 0;

    while let Some(token) = next_token(s, &mut pos) {
        match token {
            '(' => pila_paren.push('('),
            ')' => {
                if pila_paren.pop().is_none() {
                    return false;
                }
            }
            '{' => pila_llaves.push('{'),
            '}' => {
                if pila_llaves.pop().is_none() {
                    return false;
                }
            }
            _ => {} // ignoramos cualquier otro carácter
        }
    }

    // Al final, ambas pilas deben estar vacías
    pila_paren.vacia() && pila_llaves.vacia()
}

#[test]
fn test_parentesis_equilibrados_true() {
    assert_eq!(parentesis_equilibrados(" ( ) "), true);
    assert_eq!(parentesis_equilibrados(" ( ) { } "), true);
    assert_eq!(parentesis_equilibrados(" { ( ) } "), true);
}

#[test]
fn test_parentesis_equilibrados_false() {
    assert_eq!(parentesis_equilibrados(" } ( "), false);
    assert_eq!(parentesis_equilibrados(" ( ( { ( ) } { ) } ) ) "), false);
    assert_eq!(parentesis_equilibrados(" } () { "), false);
}
