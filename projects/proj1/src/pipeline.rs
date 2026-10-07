use std::env;
use std::fs::File;
use std::os::unix::fs::FileExt;
use std::sync::mpsc::sync_channel;
use std::thread;

mod hash;
use hash::sum_block_hash;
use hash::stretch_md5;
use hash::env_u64;

const BLOCK_SIZE: u64 = 1024 * 1024;
const K: u64 = 1000;

fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: [M=1048576] [K=1000] [MODE=pipe|seq] cargo run --release --bin pipeline filepath");
        return;
    }

    let file_res = File::open(args[1].clone());
    if file_res.is_err() {
        eprintln!("File not found: {}", args[1]);
        return;
    }
    let file = file_res.unwrap();

    let metadata = file.metadata();
    if metadata.is_err() {
        eprintln!("Error opening file: {}", args[1]);
        return;
    }
    let total_size = metadata.unwrap().len();
    let block_size = env_u64("M", BLOCK_SIZE);
    let k = env_u64("K", K);
    let seq = env::var("MODE").is_ok_and(|m| m == "seq");
    let num_blocks = (total_size + block_size - 1) / block_size;

    let mut buffer = vec![0u8; block_size as usize];
    let mut result = vec![0u8; 16];
    if seq {
        for block_idx in 0..num_blocks
        {
            let offset = block_idx * block_size;
            let bytes_to_read = std::cmp::min(block_size, total_size - offset) as usize;

            // read
            let target_buf = &mut buffer[..bytes_to_read];
            let err = file.read_exact_at(target_buf, offset);
            if err.is_err() {
                eprintln!("Error reading file: {}", err.unwrap_err());
                return;
            }

            // compute
            let block_hash = stretch_md5(&target_buf, k);
            sum_block_hash(&mut result, &block_hash);
        }
    } else {
        let f = &file;
        let (sender, receiver) = sync_channel(1);
        result = thread::scope(|s| {
            s.spawn(move || {
                // read thread
                for block_idx in 0..num_blocks {
                    let offset = block_idx * block_size;
                    let bytes_to_read = std::cmp::min(block_size, total_size - offset) as usize;

                    let mut buf = vec![0u8; block_size as usize];
                    let target_buf = &mut buf[..bytes_to_read];
                    f.read_exact_at(target_buf, offset)
                        .expect("Error reading file");
                    sender
                        .send((buf, bytes_to_read))
                        .expect("Failed to send through the channel");
                }
            });
            s.spawn(move || {
                // compute thread
                for (buf, len) in receiver {
                    let target_buf = &buf[..len];
                    let block_hash = stretch_md5(target_buf, k);
                    sum_block_hash(&mut result, &block_hash);
                }
                result
            })
            .join()
            .expect("Join failed")
        });
    }
    print!("{} 0x", args[1].clone());
    for byte in &result {
        print!("{:02x}", byte);
    }
    println!("");

}