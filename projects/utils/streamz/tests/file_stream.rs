use std::{fs::File, io::Write};

use rand::Rng;

use streamz::{concrete::FileStreamBuilder, sync_stream::{SyncReadStream, SyncWriteStream}};


fn create_tmp_file(content_size: usize) -> (File, RemovablePath) {
    const ALPHABET: &[u8] = "abcdefghijklmnopqrstuvwxyz0123456789".as_bytes();
    const MAX_INLINE_CONTENT_SIZE: usize = 1024;

    let mut buffer: [u8; 20] = [0; 20];
    let buffer_len = buffer.len();
    buffer[0] = b'X';
    buffer[buffer_len-4] = b'.';
    buffer[buffer_len-3] = b't';
    buffer[buffer_len-2] = b'x';
    buffer[buffer_len-1] = b't';

    let mut rng = rand::thread_rng();
    for idx in 1..(buffer_len-4) {
        let alphabet_idx = rng.gen_range(0..ALPHABET.len());
        buffer[idx] = ALPHABET[alphabet_idx];
    }
    let str_view = unsafe { core::str::from_utf8_unchecked(&buffer) };

    let full_path = std::env::temp_dir().join(str_view);
    let file_name = full_path.as_path();
    let mut file = File::create_new(file_name).unwrap();

    if content_size > 0 {
        if content_size > MAX_INLINE_CONTENT_SIZE {
            let mut vec = Vec::new();
            for _ in 0..content_size {
                let alphabet_idx = rng.gen_range(0..ALPHABET.len());
                vec.push(ALPHABET[alphabet_idx]);
            }
            file.write_all(&vec[0..content_size]).unwrap();
        }
        else
        {
            let mut write_data = [0; MAX_INLINE_CONTENT_SIZE];
            for idx in 0..content_size {
                let alphabet_idx = rng.gen_range(0..ALPHABET.len());
                write_data[idx] = ALPHABET[alphabet_idx];
            }
            file.write_all(&write_data[0..content_size]).unwrap();
        }
        file.flush().unwrap();
    }
    
    let result_file = File::open(file_name).unwrap();
    let path = String::from(full_path.to_str().unwrap());
    (result_file, RemovablePath::new(path))
}

struct RemovablePath {
    path: String,
}

impl RemovablePath {
    pub fn new(path: String) -> Self {
        Self { path: path }
    }

    pub fn path(&self) -> &str { self.path.as_str() }
}

impl Drop for RemovablePath {
    fn drop(&mut self) {
        match std::fs::remove_file(self.path.as_str()) {
            Ok(_) => {},
            Err(err) => {
                panic!("ERROR ON FILE {} CLEANUP: {}", self.path, err);
            }
        }
    }
}


#[test]
fn test_file_stream_reading() {
    let (file, _path) = create_tmp_file(4);
    let mut builder = FileStreamBuilder::default();
    builder.set_file(file);
    let mut stream = builder.build().unwrap();

    let mut buffer = [0; 4];
    let result = stream.read(&mut buffer).unwrap();
    assert_eq!(result.read_bytes(), 4);
}


#[test]
fn test_file_stream_writing() {
    let (_, rpath) = create_tmp_file(0);
    let write_buffer = [14, 1, 36, 7, 8];
    let mut read_buffer = [0; 10];
    let expected_size = 5;

    {
        let write_file = File::create(rpath.path()).unwrap();
        let mut builder = FileStreamBuilder::default();
        builder.set_file(write_file);
        let mut stream = builder.build().unwrap();
        stream.write(&write_buffer).unwrap();
        stream.flush().unwrap();
    }

    {
        let read_file = File::open(rpath.path()).unwrap();
        let mut builder = FileStreamBuilder::default();
        builder.set_file(read_file);
        let mut stream = builder.build().unwrap();
        let read_result = stream.read(&mut read_buffer).unwrap();
        assert_eq!(read_result.read_bytes(), expected_size);
    }

    assert_eq!(&read_buffer[0..expected_size], &write_buffer[0..expected_size]);
}


#[test]
fn test_file_stream_mixed() {
    const LOOP_SIZE: usize = 2000;
    let (_, rpath) = create_tmp_file(0);
    
    let mut write_content = Vec::new();
    let mut read_content = Vec::new();
    
    let mut write_stream = {
        let write_file = File::create(rpath.path()).unwrap();
        let mut builder = FileStreamBuilder::default();
        builder.set_file(write_file);
        builder.build().unwrap()
    };

    let mut read_stream = {
        let read_file = File::open(rpath.path()).unwrap();
        let mut builder = FileStreamBuilder::default();
        builder.set_file(read_file);
        builder.build().unwrap()
    };

    let mut rng = rand::thread_rng();
    let mut write_slice = [0u8; 128];
    let mut read_slice = [0u8; 128];

    for _ in 0..LOOP_SIZE {
        {
            let real_range: usize = rng.gen_range(64..128);
            for idx in 0..real_range {
                write_slice[idx] = rng.gen();
            }
            let view = &write_slice[0..real_range];
            write_stream.write(&view).unwrap();
            write_stream.flush().unwrap();
            write_content.extend_from_slice(&view);
        }

        {
            let real_range: usize = rng.gen_range(64..128);
            let view = &mut read_slice[0..real_range];
            let read_result = read_stream.read(view).unwrap();
            read_content.extend_from_slice(&view[0..read_result.read_bytes()]);
        }
    }

    let mut final_slice = [0u8; 128];
    while read_content.len() < write_content.len() {
        let real_range: usize = rng.gen_range(64..128);
        let view = &mut final_slice[0..real_range];
        let read_result = read_stream.read(view).unwrap();
        read_content.extend_from_slice(&view[0..read_result.read_bytes()]);
    }

    assert_eq!(write_content, read_content);
}
