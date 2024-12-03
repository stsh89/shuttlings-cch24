use std::net::Ipv4Addr;

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

#[cfg(test)]
mod tests {
    use super::*;

    struct V4TestData {
        a: Ipv4Addr,
        b: Ipv4Addr,
        expected: Ipv4Addr,
    }

    #[test]
    fn test_destination_math() {
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
    fn test_key_math() {
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
}
