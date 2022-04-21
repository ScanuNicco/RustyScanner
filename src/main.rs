use std::rc::Rc;
use std::fs;
use std::fs::DirEntry;
use std::path::Path;
use std::fs::File;
use std::io::{Error, Write};
use audiotags::{Tag, TagType};
use serde::Serialize;
use serde_json::Result;

#[derive(Debug, Serialize)]
struct ArtistData {
    name: String,
    albums: Vec<AlbumData>
}

#[derive(Debug, Serialize)]
struct AlbumData {
    name: String,
    songs: Vec<SongData>
}

#[derive(Debug, Serialize)]
struct SongData {
    title: String,
    file: String
}

fn main()  {
    let music_dir = "/home/nicco/Music/";
	
	let mut artists = vec![]; //Stores all data about artists. Each artist has a list of albums, and each album has a list of songs.
	//entries.sort();
    
    let fsartists: Vec<_> = fs::read_dir(music_dir).unwrap().collect(); //The direct list of folders in music_dir
	
	for artist in fsartists {
        let artist_name = artist.unwrap().file_name().to_str().unwrap().to_string();
        let artist_dir = format!("{}{}", format!("{}{}", music_dir.to_string(), artist_name), "/".to_string());
        if(Path::new(&artist_dir).is_dir()){
            let fsalbums: Vec<_> = fs::read_dir(&artist_dir).unwrap().collect(); //The direct list of folders in the artist's directory
            let mut artist_albums = vec![]; //The sanitized list of albums 
            for album in fsalbums {
                let album_name = album.unwrap().file_name().to_str().unwrap().to_string();
                let album_dir = format!("{}{}", format!("{}{}", &artist_dir, album_name), "/".to_string());
                if(Path::new(&album_dir).is_dir()){
                    let fssongs: Vec<_> = fs::read_dir(&album_dir).unwrap().collect();
                    let mut album_songs = vec![];
                    for song in fssongs {
                        let file_name = song.unwrap().file_name().to_str().unwrap().to_string();
                        let song_path = format!("{}{}", &album_dir, file_name);
                        let song_ext = Path::new(&song_path).extension().unwrap();
                        if(song_ext == "m4a" || song_ext == "mp3"){
                            let mut tag = Tag::new().read_from_path(&song_path).unwrap();
                            let song_name = tag.title().unwrap().to_string();
                            album_songs.push(SongData{ title: song_name, file: file_name});
                        }
                    }
                    artist_albums.push(AlbumData{ name: album_name, songs: album_songs});
                }
            }
            artists.push(ArtistData{ name: artist_name.to_string(), albums: artist_albums });
        }
    }
    
    let json = serde_json::to_string(&artists);
    
    
    dbg!(json);

}
