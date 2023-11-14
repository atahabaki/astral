const ASTRAL_ADDR: [u8; 4] = [127, 0, 0, 1];
const ASTRAL_PORT: u16 = 4365;

pub fn get_defaults() -> ([u8; 4], u16) {
    let addr = match std::env::var("ASTRAL_ADDR") {
        Ok(addr) => addr.to_octets(),
        Err(_) => {
            eprintln!(
                "WARN: ASTRAL_ADDR env var unset, using default: `{:?}`.",
                ASTRAL_ADDR
            );
            ASTRAL_ADDR
        }
    };
    let port = match std::env::var("ASTRAL_PORT") {
        Ok(port) => match port.parse::<u16>() {
            Ok(port) => port,
            Err(_) => {
                eprintln!(
                    "ERR: ASTRAL_PORT env var conversion failed, using default: `{}`.",
                    ASTRAL_PORT
                );
                ASTRAL_PORT
            }
        },
        Err(_) => {
            eprintln!(
                "WARN: ASTRAL_PORT env var unset, using default: `{}`.",
                ASTRAL_PORT
            );
            ASTRAL_PORT
        }
    };
    (addr, port)
}

/// Perhaps, Warp makes things easier, but I wanted to implement myself.
/// Feel free, to change or use the warp's method
trait ToOctets {
    fn to_octets(&self) -> [u8; 4];
}

impl ToOctets for String {
    fn to_octets(&self) -> [u8; 4] {
        // TODO: check if its present in the /etc/hosts like...
        let mut octets = [0_u8; 4];
        let mut dot_count = 0_u8;
        let mut buffer = "".to_owned();
        let return_default = || {
            eprintln!(
                "ERR: ASTRAL_ADDR env var conversion failed, using default: `{:?}`.",
                ASTRAL_ADDR
            );
            ASTRAL_ADDR
        };
        for c in self.chars() {
            match c {
                '.' => {
                    match buffer.parse::<u8>() {
                        Ok(octet) => {
                            octets[dot_count as usize] = octet;
                            buffer.clear();
                        }
                        Err(_) => return return_default(),
                    }
                    dot_count += 1;
                }
                '0'..='9' => buffer.push(c),
                _ => return return_default(),
            }
        }
        octets
    }
}
