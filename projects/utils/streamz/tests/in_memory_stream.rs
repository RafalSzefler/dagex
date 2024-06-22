use streamz::{
    concrete::InMemoryStreamBuilder,
    sync_stream::{SyncReadStream, SyncWriteStream},
    ReadError};


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
    assert!(matches!(stream.read(&mut buffer), Err(ReadError::StreamClosed)));
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
    assert!(matches!(stream.read(&mut read_buffer), Err(ReadError::StreamClosed)));
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
    assert!(matches!(stream.read(&mut read_buffer), Err(ReadError::StreamClosed)));
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
    assert!(matches!(stream.read(&mut read_buffer), Err(ReadError::StreamClosed)));
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

    assert!(matches!(stream.read(&mut [0, 1, 2]), Err(ReadError::StreamClosed)));
}
