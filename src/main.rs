use reqwest::Client;
use serde::Deserialize;
use chrono::{Utc, Duration};
use serde_json::Value;

#[derive(Deserialize)]
struct PhotoSearchResponse {
    photos: Photos,
}

#[derive(Deserialize)]
struct Photos {
    photo: Vec<Photo>,
}

#[derive(Deserialize)]
struct Photo {
    id: String,
}

#[derive(Deserialize)]
struct ExifResponse {
    photo: ExifPhoto,
}

#[derive(Deserialize)]
struct ExifPhoto {
    exif: Vec<ExifData>,
}

#[derive(Deserialize)]
struct ExifData {
    label: String,
    raw: RawContent,
}

#[derive(Deserialize)]
struct RawContent {
    #[serde(rename = "_content")]
    content: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = "USE_YOUR_API_KEY";
    let client = Client::new();
    let six_months_ago = (Utc::now() - Duration::weeks(26)).format("%Y-%m-%d").to_string();

    let search_url = format!("https://api.flickr.com/services/rest/?method=flickr.photos.search&api_key={}&format=json&nojsoncallback=1&sort=views&per_page=100&min_upload_date={}", api_key, six_months_ago);
    
    let resp = client.get(search_url).send().await?.json::<PhotoSearchResponse>().await?;
    let photo_ids = resp.photos.photo.into_iter().map(|photo| photo.id).collect::<Vec<_>>();

    for photo_id in photo_ids {
        let exif_url = format!("https://api.flickr.com/services/rest/?method=flickr.photos.getExif&api_key={}&photo_id={}&format=json&nojsoncallback=1", api_key, photo_id);
        let exif_resp = client.get(exif_url).send().await?;
        
        if let Ok(exif_data) = exif_resp.json::<ExifResponse>().await {
            for exif in exif_data.photo.exif {
                println!("Camera Setting for Photo ID {}: {}: {}", photo_id, exif.label, exif.raw.content);
            }
        } else {
            println!("No EXIF data available for Photo ID {}", photo_id);
        }
    }

    Ok(())
}
