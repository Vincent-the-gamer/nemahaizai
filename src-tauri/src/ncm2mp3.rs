use byteorder::ByteOrder;
use byteorder::NativeEndian;
use crypto::buffer::{BufferResult, ReadBuffer, WriteBuffer};
use crypto::{aes, blockmodes, buffer, symmetriccipher};
use id3::TagLike;
use json::object;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::mem;
use tempfile::NamedTempFile;

const AES_CORE_KEY: &[u8; 16] = b"\x68\x7A\x48\x52\x41\x6D\x73\x6F\x35\x6B\x49\x6E\x62\x61\x78\x57";
const AES_MODIFY_KEY: &[u8; 16] =
    b"\x23\x31\x34\x6C\x6A\x6B\x5F\x21\x5C\x5D\x26\x30\x55\x3C\x27\x28";

fn build_key_box(key: &[u8]) -> [u8; 256] {
    let key_len = key.len();
    let mut tmpbox: [u8; 256] = [0; 256];

    for i in 0..256 {
        tmpbox[i] = i as u8;
    }
    let mut c: u64;
    let mut last_byte: u64 = 0;

    for i in 0..256 {
        c = (tmpbox[i] as u64 + last_byte + key[(i % key_len) as usize] as u64) & 0xff;
        tmpbox.swap(i, c as usize);
        last_byte = c;
    }
    tmpbox
}

fn decrypt(
    encrypted_data: &[u8],
    key: &[u8],
) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
    let mut decryptor = aes::ecb_decryptor(aes::KeySize::KeySize128, key, blockmodes::PkcsPadding);

    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(encrypted_data);
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let result = decryptor
            .decrypt(&mut read_buffer, &mut write_buffer, true)
            .expect("Crypto decrypt error:");
        final_result.extend(
            write_buffer
                .take_read_buffer()
                .take_remaining()
                .iter()
                .map(|&i| i),
        );
        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => {}
        }
    }

    Ok(final_result)
}

pub fn process_file(
    ncm_dir: &str,
    out_dir: &str
) -> std::io::Result<String> {
    let mut ncm_dir = ncm_dir;
    if ncm_dir.ends_with("/") {
        ncm_dir = ncm_dir.strip_suffix("/").unwrap();
    }
    let ncm_path = std::path::Path::new(ncm_dir);
    let mut ulen: u32;
    let mut f = std::fs::File::open(ncm_path).expect("cannot open source file:");
    let mut buf = [0u8; mem::size_of::<u32>()];
    f.read(&mut buf)?;
    ulen = NativeEndian::read_u32(&buf);
    if ulen != 0x4e455443 {
        panic!("Not a netease music file.")
    }
    f.read(&mut buf)?;
    ulen = NativeEndian::read_u32(&buf);
    if ulen != 0x4d414446 {
        panic!("Not a netease music file.")
    }
    f.seek(SeekFrom::Current(2))?;
    let key_len: u32;
    f.read(&mut buf)?;
    key_len = NativeEndian::read_u32(&buf);
    let mut key_data: Vec<u8> = Vec::with_capacity(key_len as usize);
    key_data.resize(key_len as usize, 0);
    f.read_exact(&mut key_data)?;
    for i in 0..key_len {
        (&mut key_data)[i as usize] ^= 0x64;
    }
    let de_key_data = decrypt(&key_data, AES_CORE_KEY).expect("error decrypting key data:");
    let kbox = build_key_box(&de_key_data[17..]);
    f.read(&mut buf)?;
    ulen = NativeEndian::read_u32(&buf);
    let mut has_metadata = false;
    let mut music_info = object! {};
    if ulen > 0 {
        has_metadata = true;
        let mut modify_data: Vec<u8> = Vec::with_capacity(ulen as usize);
        modify_data.resize(ulen as usize, 0);
        f.read_exact(&mut modify_data)?;
        for i in 0..ulen {
            modify_data.as_mut_slice()[i as usize] ^= 0x63;
        }
        let data = base64::Engine::decode(&base64::engine::general_purpose::STANDARD, &modify_data[22..])
                                                 .expect("error decoding modify_data:");
        let dedata = decrypt(&data, AES_MODIFY_KEY).expect("error decrypting data:");

        music_info =
            json::parse(std::str::from_utf8(&dedata[6..]).expect("music info is not valid utf-8:"))
                .expect("error parsing json:");
    } else {
        println!(
            "{} has no metadata.",
            ncm_path.file_name().unwrap().to_str().unwrap()
        );
    }
    let s = ncm_path.file_name().unwrap().to_str().unwrap();
    let mut music_filename = s.get(0..s.len() - 4).unwrap().to_owned();
    let mut filter = std::collections::HashMap::new();
    filter.insert("\\", "＼");
    filter.insert("/", "／");
    filter.insert(":", "：");
    filter.insert("*", "＊");
    filter.insert("\"", "＂");
    filter.insert("<", "＜");
    filter.insert(">", "＞");
    filter.insert("|", "｜");
    for (k, v) in filter.iter() {
        music_filename = music_filename.replace(k, v);
    }

    let music_filename_clone = music_filename.clone();

    println!("{}", music_filename);

    f.seek(SeekFrom::Current(9))?;
    f.read(&mut buf)?;
    let img_len: u32 = NativeEndian::read_u32(&buf);
    let mut _has_cover = false;
    let mut img_data: Vec<u8> = vec![0];
    if img_len > 0 {
        _has_cover = true;
        img_data = Vec::with_capacity(img_len as usize);
        img_data.resize(img_len as usize, 0);
        f.read_exact(&mut img_data)?;
    } else {
        println!(
            "{} has no cover image.",
            ncm_path.file_name().unwrap().to_str().unwrap()
        );
    }
    let mut n: usize = 0x8000;
    let mut buffer = [0u8; 0x8000];
    let mut tmp = NamedTempFile::new()?;
    let mut format = "undefined";
    let mut filter_music_filename = music_filename;
    while n > 1 {
        n = f.read(&mut buffer)?;
        for i in 0..n {
            let j = (i + 1) & 0xff;
            // box[(box[j] + box[(box[j] + j) & 0xff]) & 0xff];
            buffer[i] ^=
                kbox[(kbox[j] as usize + kbox[(kbox[j] as usize + j) & 0xff] as usize) & 0xff];
        }
        if format == "undefined" {
            if buffer[0] == 0x49 && buffer[1] == 0x44 && buffer[2] == 0x33 {
                format = "mp3";
            } else {
                format = "flac"
            }
            filter_music_filename = filter_music_filename + "." + format;
        }
        tmp.write(&buffer)?;
    }

    let out_path_string: String;

    if out_dir.ends_with("/") {
        out_path_string = format!("{}/{}", &out_dir.strip_suffix("/").unwrap(), &filter_music_filename);
    } else {
        out_path_string = format!("{}/{}", &out_dir, &filter_music_filename);
    }
    
    std::fs::copy(
        tmp.into_temp_path(),
        std::path::Path::new(&out_path_string),
    )?;

    drop(f);
    if has_metadata {
        let mut mimetype = "";
        if _has_cover {
            let png: Vec<u8> = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
            if png == &img_data[..8] {
                mimetype = "image/png";
            } else {
                mimetype = "image/jpeg";
            }
        }
        let music_name = music_info["musicName"].as_str().unwrap();
        let album = music_info["album"].as_str().unwrap();
        let artist = &music_info["artist"];
        let _bitrate = music_info["bitrate"].as_i64().unwrap();
        let _duration = music_info["duration"].as_i64().unwrap();
        if format == "mp3" {
            let mut tag = id3::Tag::read_from_path(std::path::Path::new(&out_path_string))
                .unwrap_or(id3::Tag::new());
            tag.set_title(music_name);
            tag.set_album(album);
            let mut artists = String::from(artist[0][0].as_str().unwrap());
            for i in 1..artist.len() {
                artists += "/";
                artists += artist[i][0].as_str().unwrap();
            }
            tag.set_artist(artists);
            if _has_cover {
                let picture = id3::frame::Picture {
                    mime_type: mimetype.to_owned(),
                    picture_type: id3::frame::PictureType::CoverFront,
                    description: String::new(),
                    data: img_data,
                };
                tag.add_frame(picture);
            }
            tag.write_to_path(
                std::path::Path::new(&out_path_string),
                id3::Version::Id3v24,
            )
            .expect("error writing MP3 file:");
        } else if format == "flac" {
            // flac
            let mut tag =
                metaflac::Tag::read_from_path(std::path::Path::new(&out_path_string))
                    .expect("error reading flac file:");
            let c = tag.vorbis_comments_mut();
            c.set_title(vec![music_name]);
            c.set_album(vec![album]);
            let mut artists: Vec<String> = Vec::new();
            for i in 0..artist.len() {
                artists.push(artist[i][0].as_str().unwrap().to_string());
            }
            c.set_artist(artists);
            if _has_cover {
                tag.add_picture(mimetype, metaflac::block::PictureType::CoverFront, img_data);
            }
            tag.write_to_path(std::path::Path::new(&out_path_string))
                .expect("error writing flac file:");
        }
    }
    Ok(music_filename_clone)
}