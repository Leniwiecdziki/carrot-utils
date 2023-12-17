#![allow(dead_code)]
use std::fs;
use std::path::Path;
use std::os::unix::fs::MetadataExt;
use std::os::unix::fs::FileTypeExt;
use std::os::unix::fs::PermissionsExt;

pub fn ftype(path:&Path) -> std::io::Result<String> {
    match fs::symlink_metadata(path) {
        Err(e) => Err(e),
        Ok(e) => Ok(
            if e.is_dir() { String::from("Directory") }
            else if e.is_file() { String::from("File") }
            else if e.is_symlink() { String::from("Symlink") }
            else if e.file_type().is_block_device() { String::from("Block") }
            else if e.file_type().is_char_device() { String::from("Char") }
            else if e.file_type().is_fifo() { String::from("Fifo") }
            else if e.file_type().is_socket() { String::from("Socket") }
            else { String::from("Unknown") }
        )
    }
}

pub fn size(path:&Path) -> Result<u64, std::io::Error> {
    match fs::symlink_metadata(path) {
        Err(e) => Err(e),
        Ok(e) => Ok(e.len()) }
}

pub fn perms(path:&Path) -> Result<(u32, u32, u32, u32), std::io::Error> {
    match fs::symlink_metadata(path) {
        Err(e) => Err(e),
        Ok(e) => {
            let additional:u32 = format!("{:o}", e.permissions().mode()).chars().nth_back(3).unwrap().to_digit(10).unwrap();
            let user:u32 = format!("{:o}", e.permissions().mode()).chars().nth_back(2).unwrap().to_digit(10).unwrap();
            let group:u32 = format!("{:o}", e.permissions().mode()).chars().nth_back(1).unwrap().to_digit(10).unwrap();
            let other:u32 = format!("{:o}", e.permissions().mode()).chars().nth_back(0).unwrap().to_digit(10).unwrap();
            Ok((additional, user, group, other))
        },
    }
}

pub fn uid(path:&Path) -> Result<u32, std::io::Error> {
    match fs::symlink_metadata(path) {
        Err(e) => Err(e),
        Ok(e) => Ok(e.uid()) }
}

pub fn gid(path:&Path) -> Result<u32, std::io::Error> {
    match fs::symlink_metadata(path) {
        Err(e) => Err(e),
        Ok(e) => Ok(e.gid()) }
}

pub fn inode(path:&Path) -> Result<u64, std::io::Error> {
    match fs::symlink_metadata(path) {
        Err(e) => Err(e),
        Ok(e) => Ok(e.ino()) }
}

pub fn hlinks(path:&Path) -> Result<u64, std::io::Error> {
    match fs::symlink_metadata(path) {
        Err(e) => Err(e),
        Ok(e) => Ok(e.nlink()) }
}

pub fn atime(path:&Path) -> Result<i64, std::io::Error> {
    match fs::symlink_metadata(path) {
        Err(e) => Err(e),
        Ok(e) => Ok(e.atime()) }
}
pub fn ctime(path:&Path) -> Result<i64, std::io::Error> {
    match fs::symlink_metadata(path) {
        Err(e) => Err(e),
        Ok(e) => Ok(e.ctime()) }
}
pub fn mtime(path:&Path) -> Result<i64, std::io::Error> {
    match fs::symlink_metadata(path) {
        Err(e) => Err(e),
        Ok(e) => Ok(e.mtime()) }
}