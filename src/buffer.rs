fn get_shortest_length(a: &Vec<u8>, b: &Vec<u8>) -> usize {
    if a.len() > b.len() {
        b.len()
    } else {
        a.len()
    }
}

fn match_size(a: &mut Vec<u8>, b: &Vec<u8>) -> () {
    if a.len() < b.len() {
        a.resize(b.len(), 0);
    }
}

pub fn collect_usize(a: &Vec<u8>) -> usize {
    let mut out = 0;

    for i in 0..a.len() {
        out += (a[i] as usize) << i * 8;
    }

    out
}

pub fn collect_isize(a: &Vec<u8>) -> isize {
    todo!();
}

pub fn xor(a: &mut Vec<u8>, b: &Vec<u8>) -> () {
    match_size(a, b);

    for i in 0..get_shortest_length(&a, &b) {
        a[i] ^= b[i];
    }
}

pub fn and(a: &mut Vec<u8>, b: &Vec<u8>) {
    match_size(a, b);

    for i in 0..get_shortest_length(&a, &b) {
        a[i] &= b[i];
    }
}

pub fn or(a: &mut Vec<u8>, b: &Vec<u8>) {
    match_size(a, b);

    for i in 0..get_shortest_length(&a, &b) {
        a[i] |= b[i];
    }
}

pub fn not(a: &mut Vec<u8>) {
    for byte in a {
        *byte = !*byte;
    }
}

pub fn shift(a: &mut Vec<u8>, amount: isize) {
    let byte_position = (amount.abs() / 8) as usize;
    let bit_position = (amount.abs() % 8) as usize;

    if amount > 0 {
        for i in 0..a.len() - byte_position {
            a[i] = a[i + byte_position] << bit_position;
        }
    } else {
        for i in (byte_position..a.len()).rev() {
            a[i] = a[i - byte_position] >> bit_position;
        }
    }
}

pub fn add(a: &mut Vec<u8>, b: &Vec<u8>) {
    let shortest = get_shortest_length(a, b);
    let mut overflow;

    for i in 0..shortest {}
}

pub fn sub(a: &mut Vec<u8>, b: &Vec<u8>) {
    let shortest = get_shortest_length(a, b);

    for i in 0..shortest {
        let overflow;
        (a[i], overflow) = a[i].overflowing_sub(b[i]);
        a[i + 1] -= overflow as u8;
    }
}

pub fn neg(a: &mut Vec<u8>) {
    for i in 0..a.len() {
        a[i] = !a[i];
    }
    add(a, &vec![1]);
}

pub fn mul(a: &Vec<u8>, b: &Vec<u8>) -> Vec<u8> {
    let mut c: Vec<u8> = Vec::with_capacity(a.len() + b.len());
    c.resize(a.len() + b.len(), 0);

    for i in 0..a.len() {
        for j in 0..b.len() {
            let result = c[i + j] as usize + a[i] as usize * b[j] as usize;
            c[i + j] = (result % 256) as u8;
            c[i + j + 1] = (result / 256) as u8;
        }
    }

    c
}
