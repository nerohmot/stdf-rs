use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};


#[derive(Debug, PartialEq)]
pub enum Endian {
    Little,
    Big,
}

/// Calculates the number of bytes remaining in the file after the current file pointer position.
///
/// # Arguments
///
/// * `file` - A mutable reference to the file for which to calculate the remaining bytes.
///
/// # Returns
///
/// This function returns an `io::Result<u64>` containing the number of bytes remaining after the
/// current file pointer position. If an error occurs during seeking, the error is returned.
fn bytes_after_filepointer(file: &mut File) -> io::Result<u64> {
    let current_position = file.seek(SeekFrom::Current(0))?;
    let total_length = file.seek(SeekFrom::End(0))?;
    file.seek(SeekFrom::Start(current_position))?;
    Ok(total_length - current_position)
}


 pub fn get_endianness_from_file(file: &mut File) -> io::Result<Option<Endian>> {
    let current_position = file.seek(SeekFrom::Current(0))?;
    file.seek(SeekFrom::Start(0))?;
    if bytes_after_filepointer(file)? >= 6 {
        let mut rec_len: [u8;2] = [0, 0];
        file.read_exact(&mut rec_len)?;
        if u16::from_le_bytes(rec_len) == 2 {
            file.seek(SeekFrom::Start(current_position))?;
            return Ok(Some(Endian::Little));
        } else { 
            file.seek(SeekFrom::Start(current_position))?;
            return Ok(Some(Endian::Big));
        }
    } else {
        file.seek(SeekFrom::Start(current_position))?;
        return Ok(None);
    }
}


// /// Returns the next available record with the given endianness.
// /// 
// /// # Arguments
// /// 
// /// * `file` - A mutable reference to the file to read the record from
// /// * `endian` - The endianness to use for the calculation of the REC_LEN 
// /// 
// /// # Returns
// /// 
// /// This function returns an `io::Result<Option<Vec<u8>>>` containing the record in a vector
// /// or None. If an error occurs during this process, the error is returned.
// /// 
// pub fn read_record_from_file(mut file: File, endian: Endian) -> io::Result<Option<Vec<u8>>> {
//     let initial_position = file.seek(SeekFrom::Current(0))?;
//     if bytes_after_filepointer(file)? >= 4 {  // header bytes present
//         let mut record = vec![0; 2];
//         file.read_exact(&mut record)?;
//         let rec_len = match endian {
//             Endian::Little => {
//                 let mut padded_bytes = [0u8; 8];
//                 padded_bytes[..2].copy_from_slice(&record);
//                 u64::from_le_bytes(padded_bytes)
//             }
//             Endian::Big => {
//                 let mut padded_bytes = [0u8; 8];
//                 padded_bytes[..2].copy_from_slice(&record);
//                 u64::from_be_bytes(padded_bytes)
//             }
//         };
    
//         let mut id = vec![0;2];
//         file.read_exact(&mut id)?;
//         record.extend(id);

//         if bytes_after_filepointer(file) >= rec_len { // tail bytes present
//             let mut tail = vec![0; rec_len];
//             file.read_exact(&mut tail)?;
//             record.extend(tail);
//             Ok(Some(record)) 
//         } else { // no tail bytes present (yet) --> reset file pointer
//             file.seek(SeekFrom::Start(initial_position))?;
//             Ok(None) 
//         }
//     } else { // no header bytes present --> reset file pointer
//         file.seek(SeekFrom::Start(initial_position))?;
//         Ok(None) // not even a header
//     }
// }


// /// Writes a record to a file in the given endian-nes.
// /// 
// /// # Arguments
// /// 
// /// * `file` - A mutable reference to the file to write the bytes to
// /// * `record` - The record (in whatever endianness)
// pub fn write_record_to_file(mut file: File, record:Vec<u8>) -> io::Result<()> {
//     Ok(())
// }

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::tempfile;

    #[test]
    fn test_bytes_after_filepointer() {
        let mut file = tempfile().unwrap();
        write!(file, "Hello, world!").unwrap();
        file.seek(SeekFrom::Start(7)).unwrap(); // Move the file pointer to position 7
        let remaining_bytes = bytes_after_filepointer(&mut file).unwrap();
        assert_eq!(remaining_bytes, 6); // "world!" has 6 bytes
    }

    #[test]
    fn test_get_endianness_from_file() {
        let mut file = tempfile().unwrap();

        // File that is too small to even hold a FAR
        file.write_all(&[0x02, 0x00]).unwrap();
        file.seek(SeekFrom::Start(0)).unwrap();
        let endianness = get_endianness_from_file(&mut file).unwrap();
        assert_eq!(endianness, None);

        // Write a FAR record with REC_LEN = 2 (little-endian)
        file.write_all(&[0x02, 0x00, 0x00, 0x0A, 0x00, 0x00]).unwrap();
        file.seek(SeekFrom::Start(0)).unwrap();
        let endianness = get_endianness_from_file(&mut file).unwrap();
        assert_eq!(endianness, Some(Endian::Little));

        // Write a FAR record with REC_LEN = 2 (big-endian)
        file.seek(SeekFrom::Start(0)).unwrap();
        file.write_all(&[0x00, 0x02, 0x00, 0x0A, 0x00, 0x00]).unwrap();
        file.seek(SeekFrom::Start(0)).unwrap();
        let endianness = get_endianness_from_file(&mut file).unwrap();
        assert_eq!(endianness, Some(Endian::Big));
    }
}