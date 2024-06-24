use std::ops::Range;

use array::Array;
use rand::Rng;
use streamz::{
    concrete::InMemoryStreamBuilder,
    sync_stream::{SyncReadStream, SyncWriteStream}};


#[test]
fn test_in_memory_stream_empty() {
    let mut stream = InMemoryStreamBuilder::default()
        .build().unwrap();

    let write_result = stream.write(&[]);
    assert!(write_result.is_ok());
    let mut buffer: [u8; 0] = [];
    assert!(stream.read(&mut buffer).is_ok());
}


#[test]
fn test_in_memory_stream_basic() {
    let mut stream = InMemoryStreamBuilder::default()
        .build().unwrap();

    let write_result = stream.write(&[1, 2, 3]);
    assert!(write_result.is_ok());
    let mut buffer: [u8; 10] = [0; 10];
    let read_result = stream.read(&mut buffer).unwrap();
    assert_eq!(read_result.read_bytes(), 3);
    assert_eq!(&buffer[0..3], &[1, 2, 3]);
    assert_eq!(stream.read(&mut [1]).unwrap().read_bytes(), 0);
}


#[test]
fn test_in_memory_stream_big_read() {
    let mut stream = InMemoryStreamBuilder::default().build().unwrap();

    let write_buffer: [u8; 11] = [101; 11];
    stream.write(&write_buffer).unwrap();
    
    let mut read_buffer: [u8; 33] = [0; 33];
    let read_result = stream.read(&mut read_buffer).unwrap();
    assert_eq!(read_result.read_bytes(), 11);
    assert_eq!(&read_buffer[0..11], &write_buffer);
    assert_eq!(stream.read(&mut [1]).unwrap().read_bytes(), 0);
}


#[test]
fn test_in_memory_stream_small_buffer() {
    let mut builder = InMemoryStreamBuilder::default();
    builder.set_buffer_size(10);
    let mut stream = builder.build().unwrap();

    let write_buffer: [u8; 11] = [101; 11];
    stream.write(&write_buffer).unwrap();
    stream.write(&write_buffer).unwrap();
    stream.write(&write_buffer).unwrap();
    
    let mut read_buffer: [u8; 33] = [0; 33];
    let read_result = stream.read(&mut read_buffer).unwrap();
    assert_eq!(read_result.read_bytes(), 33);
    assert_eq!(&read_buffer[0..11], &write_buffer);
    assert_eq!(&read_buffer[11..22], &write_buffer);
    assert_eq!(&read_buffer[22..33], &write_buffer);
    assert_eq!(stream.read(&mut [1]).unwrap().read_bytes(), 0);
}


#[test]
fn test_in_memory_stream_big_buffer() {
    let mut builder = InMemoryStreamBuilder::default();
    builder.set_buffer_size(1024);
    let mut stream = builder.build().unwrap();

    let write_buffer: [u8; 11] = [101; 11];
    stream.write(&write_buffer).unwrap();
    stream.write(&write_buffer).unwrap();
    stream.write(&write_buffer).unwrap();
    
    let mut read_buffer: [u8; 33] = [0; 33];
    let read_result = stream.read(&mut read_buffer).unwrap();
    assert_eq!(read_result.read_bytes(), 33);
    assert_eq!(&read_buffer[0..11], &write_buffer);
    assert_eq!(&read_buffer[11..22], &write_buffer);
    assert_eq!(&read_buffer[22..33], &write_buffer);
    assert_eq!(stream.read(&mut [1]).unwrap().read_bytes(), 0);
}


#[test]
fn test_in_memory_stream_big_loop() {
    let mut builder = InMemoryStreamBuilder::default();
    builder.set_buffer_size(10);
    let mut stream = builder.build().unwrap();

    for _loop_counter in 0..100 {
        let write_buffer_1: [u8; 11] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        stream.write(&write_buffer_1).unwrap();
        let write_buffer_2: [u8; 11] = [41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51];
        stream.write(&write_buffer_2).unwrap();
        let write_buffer_3: [u8; 11] = [61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71];
        stream.write(&write_buffer_3).unwrap();
        
        let mut read_buffer: [u8; 33] = [0; 33];
        let read_result = stream.read(&mut read_buffer).unwrap();
        assert_eq!(read_result.read_bytes(), 33);
        assert_eq!(&read_buffer[0..11], &write_buffer_1);
        assert_eq!(&read_buffer[11..22], &write_buffer_2);
        assert_eq!(&read_buffer[22..33], &write_buffer_3);
    }

    assert_eq!(stream.read(&mut [1]).unwrap().read_bytes(), 0);
}


#[test]
fn test_randomized() {
    const LOOP_SIZE: usize = 2000;
    const STREAM_BUFFER: usize = 14;
    const ARRAY_LENGTH: Range<usize> = (STREAM_BUFFER - 10)..(STREAM_BUFFER + 10);

    fn generate_array(len_range: Range<usize>) -> Array<u8> {
        let mut rng = rand::thread_rng();
        let length = rng.gen_range(len_range);
        Array::new(length)
    }

    fn generate_array_with_content(len_range: Range<usize>) -> Array<u8> {
        let mut arr = generate_array(len_range);
        let mut rng = rand::thread_rng();
        let slice = arr.as_slice_mut();
        let length = slice.len();
        for idx in 0..length {
            slice[idx] = rng.gen();
        }
        arr
    }

    let mut initial_stream_buffer_size = STREAM_BUFFER;

    for _ in 0..5 {
        let mut builder = InMemoryStreamBuilder::default();
        builder.set_buffer_size(initial_stream_buffer_size);
        let mut stream = builder.build().unwrap();

        let mut total_write = Vec::<u8>::with_capacity(LOOP_SIZE * STREAM_BUFFER);
        let mut total_read = Vec::<u8>::with_capacity(LOOP_SIZE * STREAM_BUFFER);

        for _ in 0..LOOP_SIZE {
            let write = generate_array_with_content(ARRAY_LENGTH);
            let write_slice = write.as_slice();
            let write_len = write_slice.len();
            stream.write(write_slice).unwrap();
            total_write.extend_from_slice(write_slice);

            let mut read = generate_array(ARRAY_LENGTH);
            let read_buffer = read.as_slice_mut();
            let result = stream.read(read_buffer).unwrap();
            let read_bytes = result.read_bytes();
            assert!(read_bytes >= core::cmp::min(read_buffer.len(), write_len));

            total_read.extend_from_slice(&read_buffer[0..read_bytes]);
        }

        let mut final_read = generate_array(ARRAY_LENGTH);
        let final_read_slice = final_read.as_slice_mut();
        loop {
            let result = stream.read(final_read_slice).unwrap();
            let read_bytes = result.read_bytes();
            if read_bytes == 0 {
                break;
            }
            total_read.extend_from_slice(&final_read_slice[0..read_bytes]);
        }

        assert_eq!(total_write, total_read);
        initial_stream_buffer_size *= 10;
    }
}


#[test]
fn test_in_memory_stream_iteration() {
    let mut builder = InMemoryStreamBuilder::default();
    builder.set_buffer_size(5);
    let mut stream = builder.build().unwrap();

    let write_buffer_1 = [1, 2, 3, 4];
    stream.write(&write_buffer_1).unwrap();
    let write_buffer_2 = [5, 6, 7];
    stream.write(&write_buffer_2).unwrap();

    let iter_pages = stream.iter_pages();
    let iter_len = iter_pages.len();
    assert_eq!(iter_len, 2);
    let current_data = Vec::from_iter(iter_pages);
    assert_eq!(current_data.len(), 2);
    assert_eq!(current_data[0], &[1, 2, 3, 4, 5]);
    assert_eq!(current_data[1], &[6, 7]);

    stream.read(&mut [0, 0, 0]).unwrap();
    let iter_pages = stream.iter_pages();
    let iter_len = iter_pages.len();
    assert_eq!(iter_len, 2);
    let current_data = Vec::from_iter(iter_pages);
    assert_eq!(current_data.len(), 2);
    assert_eq!(current_data[0], &[4, 5]);
    assert_eq!(current_data[1], &[6, 7]);

    stream.reset();
    let iter_pages = stream.iter_pages();
    let iter_len = iter_pages.len();
    assert_eq!(iter_len, 0);
    let current_data = Vec::from_iter(iter_pages);
    assert_eq!(current_data.len(), 0);
}


#[test]
fn test_in_memory_stream_iteration_2() {
    let mut builder = InMemoryStreamBuilder::default();
    builder.set_buffer_size(5);
    let mut stream = builder.build().unwrap();

    let write_buffer_1 = [1, 2, 3, 4];
    stream.write(&write_buffer_1).unwrap();
    let write_buffer_2 = [5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    stream.write(&write_buffer_2).unwrap();

    let iter_pages = stream.iter_pages();
    let iter_len = iter_pages.len();
    assert_eq!(iter_len, 4);
    let current_data = Vec::from_iter(iter_pages);
    assert_eq!(current_data.len(), 4);
    assert_eq!(current_data[0], &[1, 2, 3, 4, 5]);
    assert_eq!(current_data[1], &[6, 7, 8, 9, 10]);
    assert_eq!(current_data[2], &[11, 12, 13, 14, 15]);
    assert_eq!(current_data[3], &[16]);

    stream.read(&mut [0, 0, 0]).unwrap();
    let iter_pages = stream.iter_pages();
    let iter_len = iter_pages.len();
    assert_eq!(iter_len, 4);
    let current_data = Vec::from_iter(iter_pages);
    assert_eq!(current_data.len(), 4);
    assert_eq!(current_data[0], &[4, 5]);
    assert_eq!(current_data[1], &[6, 7, 8, 9, 10]);
    assert_eq!(current_data[2], &[11, 12, 13, 14, 15]);
    assert_eq!(current_data[3], &[16]);

    stream.reset();
    let iter_pages = stream.iter_pages();
    let iter_len = iter_pages.len();
    assert_eq!(iter_len, 0);
    let current_data = Vec::from_iter(iter_pages);
    assert_eq!(current_data.len(), 0);
}
