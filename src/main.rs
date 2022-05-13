#![allow(non_snake_case)]
#![allow(unused_parens)]

use std::fs;
use std::path::Path;
use std::io::Write;
use std::env;
use audiotags::Tag;
use serde::Serialize;
use image::io::Reader as ImageReader;

#[derive(Debug, Serialize)]
struct ArtistData {
    name: String,
    albums: Vec<AlbumData>
}

impl ArtistData {
    fn name_without_the(&self) -> &str {
        self.name.trim_start_matches("The ")
    }
}

#[derive(Debug, Serialize)]
struct AlbumData {
    name: String,
    artwork: String,
    songs: Vec<SongData>
}

#[derive(Debug, Serialize)]
struct SongData {
    title: String,
    file: String
}

fn main()  {
    let music_dir = env::args().nth(1).expect("expected path");
    
    fs::create_dir_all("album_artwork");
	
	let mut artists = vec![]; //Stores all data about artists. Each artist has a list of albums, and each album has a list of songs.
	//entries.sort();
    
    let fsartists: Vec<_> = fs::read_dir(&music_dir).unwrap().collect(); //The direct list of folders in music_dir
	
	for artist in fsartists { 
        //For each artist
        let artist_name = artist.unwrap().file_name().to_str().unwrap().to_string();
        println!("{}", &artist_name);
        let artist_dir = format!("{}{}", format!("{}{}", music_dir.to_string(), artist_name), "/".to_string());
        if(Path::new(&artist_dir).is_dir()){ //Make sure item is a directory
            fs::create_dir_all("album_artwork/".to_owned() + &artist_name);
            let fsalbums: Vec<_> = fs::read_dir(&artist_dir).unwrap().collect(); //The direct list of folders in the artist's directory
            let mut artist_albums = vec![]; //The sanitized list of albums 
            for album in fsalbums { 
                //For Each Album
                let album_name = album.unwrap().file_name().to_str().unwrap().to_string();
                let album_dir = format!("{}{}", format!("{}{}", &artist_dir, album_name), "/".to_string());
                if(Path::new(&album_dir).is_dir()){ //Make sure item is a directory
                    let possiblePath = "album_artwork/".to_owned() + &artist_name + "/" + &album_name + ".png";
                    let mut gotArt = std::path::Path::new(&possiblePath).exists();
                    let mut albumArtPath = "placeholder.jpg";
                    if(gotArt){
                        albumArtPath = &possiblePath;
                    }
                    let fssongs: Vec<_> = fs::read_dir(&album_dir).unwrap().collect();
                    let mut album_songs = vec![];
                    for song in fssongs { 
                        //For Each Song
                        let file_name = song.unwrap().file_name().to_str().unwrap().to_string();
                        let song_path = format!("{}{}", &album_dir, file_name);
                        let song_ext = Path::new(&song_path).extension().unwrap();
                        if(song_ext == "m4a" || song_ext == "mp3"){
                            let mut song_name = file_name.clone();
                            if let Ok(tag) = Tag::new().read_from_path(&song_path) {
                                song_name = tag.title().unwrap_or(&file_name).to_string();
                                if(!gotArt){ //To account for a weird edge-case in my music library, check every song for album art instead of only one. If a song with album art is found, there is no need to continue checking.
                                    if let Some(possible_artwork) = tag.album_cover() {
                                        let album_artwork = possible_artwork;
                                        //dbg!(album_artwork.mime_type);
                                        let imgType = match album_artwork.mime_type {
                                            audiotags::MimeType::Png => image::ImageFormat::Png,
                                            audiotags::MimeType::Jpeg => image::ImageFormat::Jpeg,
                                            audiotags::MimeType::Tiff => image::ImageFormat::Tiff,
                                            audiotags::MimeType::Bmp => image::ImageFormat::Bmp,
                                            audiotags::MimeType::Gif => image::ImageFormat::Gif
                                        };
                                        let img = image::load_from_memory_with_format(album_artwork.data, imgType).unwrap();
                                        img.save(&possiblePath);
                                        gotArt = true;
                                        albumArtPath = &possiblePath.as_str();
                                    }
                                }
                            }
                            //dbg!(album_artwork);
                            album_songs.push(SongData{ title: song_name.to_string(), file: file_name});
                        }
                    }
                    album_songs.sort_by(|a, b| a.title.cmp(&b.title));
                    artist_albums.push(AlbumData{ name: album_name, songs: album_songs, artwork: albumArtPath.to_string()});
                }
            }
            artists.push(ArtistData{ name: artist_name.to_string(), albums: artist_albums });
        }
    }
    
    artists.sort_by(|a, b| a.name_without_the().cmp(b.name_without_the()));
    
    let json = serde_json::to_string(&artists);
    
    let mut fileRef = std::fs::File::create("music.json").expect("create failed");
   
    fileRef.write_all(json.unwrap().as_bytes()).expect("write failed");

}
