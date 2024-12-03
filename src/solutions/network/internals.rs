use std::net::{Ipv4Addr, Ipv6Addr};

pub fn add_v4_addresses(a: Ipv4Addr, b: Ipv4Addr) -> Ipv4Addr {
    let a = a.octets();
    let b = b.octets();

    Ipv4Addr::new(
        a[0].wrapping_add(b[0]),
        a[1].wrapping_add(b[1]),
        a[2].wrapping_add(b[2]),
        a[3].wrapping_add(b[3]),
    )
}

pub fn sub_v4_address(a: Ipv4Addr, b: Ipv4Addr) -> Ipv4Addr {
    let a = a.octets();
    let b = b.octets();

    Ipv4Addr::new(
        a[0].wrapping_sub(b[0]),
        a[1].wrapping_sub(b[1]),
        a[2].wrapping_sub(b[2]),
        a[3].wrapping_sub(b[3]),
    )
}

pub fn xor_v6_addresses(a: Ipv6Addr, b: Ipv6Addr) -> Ipv6Addr {
    let octets: Vec<u8> = a
        .octets()
        .iter()
        .zip(b.octets())
        .map(|(a, b)| a ^ b)
        .collect();

    Ipv6Addr::new(
        ((octets[0] as u16) << 8) | octets[1] as u16,
        ((octets[2] as u16) << 8) | octets[3] as u16,
        ((octets[4] as u16) << 8) | octets[5] as u16,
        ((octets[6] as u16) << 8) | octets[7] as u16,
        ((octets[8] as u16) << 8) | octets[9] as u16,
        ((octets[10] as u16) << 8) | octets[11] as u16,
        ((octets[12] as u16) << 8) | octets[13] as u16,
        ((octets[14] as u16) << 8) | octets[15] as u16,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    struct V4TestData {
        a: Ipv4Addr,
        b: Ipv4Addr,
        expected: Ipv4Addr,
    }

    struct V6TestData {
        a: Ipv6Addr,
        b: Ipv6Addr,
        expected: Ipv6Addr,
    }

    #[test]
    fn test_add_v4_addresses() {
        let test_data = vec![
            V4TestData {
                a: Ipv4Addr::new(10, 0, 0, 0),
                b: Ipv4Addr::new(1, 2, 3, 255),
                expected: Ipv4Addr::new(11, 2, 3, 255),
            },
            V4TestData {
                a: Ipv4Addr::new(128, 128, 33, 0),
                b: Ipv4Addr::new(255, 0, 255, 33),
                expected: Ipv4Addr::new(127, 128, 32, 33),
            },
        ];

        for data in test_data {
            let V4TestData { a, b, expected } = data;

            let got = add_v4_addresses(a, b);

            assert_eq!(
                got, expected,
                "{} + {}, expected: {}, got: {}",
                a, b, expected, got
            );
        }
    }

    #[test]
    fn test_sub_v4_addresses() {
        let test_data = vec![
            V4TestData {
                a: Ipv4Addr::new(11, 2, 3, 255),
                b: Ipv4Addr::new(10, 0, 0, 0),
                expected: Ipv4Addr::new(1, 2, 3, 255),
            },
            V4TestData {
                a: Ipv4Addr::new(127, 128, 32, 33),
                b: Ipv4Addr::new(128, 128, 33, 0),
                expected: Ipv4Addr::new(255, 0, 255, 33),
            },
        ];

        for data in test_data {
            let V4TestData { a, b, expected } = data;

            let got = sub_v4_address(a, b);

            assert_eq!(
                got, expected,
                "{} - {}, expected: {}, got: {}",
                a, b, expected, got
            );
        }
    }

    #[test]
    fn test_xor_v6_addresses() {
        let test_data = vec![
            V6TestData {
                a: "fe80::1".parse().unwrap(),
                b: "5:6:7::3333".parse().unwrap(),
                expected: "fe85:6:7::3332".parse().unwrap(),
            },
            V6TestData {
                a: "5555:ffff:c:0:0:c:1234:5555".parse().unwrap(),
                b: "aaaa::aaaa".parse().unwrap(),
                expected: "ffff:ffff:c::c:1234:ffff".parse().unwrap(),
            },
        ];

        for data in test_data {
            let V6TestData { a, b, expected } = data;

            let got = xor_v6_addresses(a, b);

            assert_eq!(
                got, expected,
                "{} xor {}, expected: {}, got: {}",
                a, b, expected, got
            );
        }
    }
}
