use sha2::{Digest, Sha256};
use std:: fmt;
use std::time::(SystemTime,UNIX_EPOCH);
use std::thread;
use std::time::Duration

// define block_chain difficulty

const DIFFICULTY: usize = 2