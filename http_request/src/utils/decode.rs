use std::io;
/// This is going to decode the raw data the program will get from the itm.txt file.
///
/// #Arguments
/// raw_data - decode function is going to take the raw data.
///
/// #Return
/// Vec<u8> this function is going to return a vector of u8.
pub fn decode<R: io::Read>(mut raw_data: R) -> io::Result<Vec<u8>> {
    let mut output = vec![];
    loop {
        let mut len = 0u8;
        raw_data.read_exact(std::slice::from_mut(&mut len))?;
        let len = 1 << (len - 1);
        let mut buf = vec![0; len];
        let _res = raw_data.read_exact(&mut buf);
        if buf == b"\0" {
            break;
        }
        output.extend(buf);
    }
    Ok(output)
}

#[cfg(test)]
mod test {
    use crate::utils::decode::decode;
    #[test]
    fn test_success() {
        let data = "{ x: 579, y: -197 , z: -485 }\0";
        let decoded_data = decode(data.as_bytes()).unwrap();

        assert_eq!(
            std::str::from_utf8(&decoded_data).unwrap(),
            "{ x: 579, y: -197 , z: -485 }"
        );
    }
    #[test]
    fn test_failure() {
        let data = "{ x: 579, y: -197 , z: -485 }\0";
        let decoded_data = decode(data.as_bytes()).unwrap();

        assert_eq!(
            std::str::from_utf8(&decoded_data).unwrap(),
            "{y: -197 , z: -485 }"
        );
    }
}
