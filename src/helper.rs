use std::fs::read_to_string;
use std::fs::File;
use std::io::Write;
use std::sync::Arc;

use once_cell::sync::Lazy;
use tokio::sync::RwLock;

use crate::structs::tags::Tag;

static EN_TAGS: Lazy<Arc<RwLock<Vec<Tag>>>> = Lazy::new(|| Arc::new(RwLock::new(Vec::new())));

pub async fn refresh_tags(lang: &str) -> Vec<Tag> {
    let tags = {
        if lang == "en" {
            Arc::clone(&EN_TAGS)
        } else {
            unreachable!()
        }
    };
    let mut tags = tags.write().await;
    tags.clear();
    let json = read_to_string(format!("tags/{}.json", lang)).unwrap();
    let data: Vec<Tag> = serde_json::from_str(&json).unwrap();
    tags.extend(data.clone());
    tags.clone()
}

pub async fn get_tags(lang: &str) -> Vec<Tag> {
    let tags = {
        if lang == "en" {
            Arc::clone(&EN_TAGS)
        } else {
            unreachable!()
        }
    };
    let tags = tags.read().await;
    let data = tags.clone();
    if data.is_empty() {
        drop(tags);
        let data = refresh_tags(lang).await;
        return data;
    }
    data.clone()
}

pub async fn add_tag(tag: Tag, lang: &str) {
    let og_en_tags = Arc::clone(&EN_TAGS);
    let mut en_tags = og_en_tags.read().await;
    if en_tags.is_empty() {
        drop(en_tags);
        refresh_tags(lang).await;
        en_tags = og_en_tags.read().await;
    }
    let mut vec = en_tags.clone();
    vec.push(tag.clone());
    let mut file = File::create("tags/en.json").unwrap();
    file.write_all(serde_json::to_string(&vec).unwrap().as_bytes())
        .unwrap();
    drop(en_tags);
    refresh_tags(lang).await;
}

pub async fn remove_tag(key: String, lang: &str) -> Result<(), String> {
    let og_en_tags = Arc::clone(&EN_TAGS);
    let mut en_tags = og_en_tags.read().await;
    if en_tags.is_empty() {
        drop(en_tags);
        refresh_tags(lang).await;
        en_tags = og_en_tags.read().await;
    }
    let mut vec = en_tags.clone();
    if vec.len() == 1 {
        return Err("Cannot remove last tag".to_string());
    }
    if !vec.iter().any(|t| t.key == key) {
        return Err("Tag not found".to_string());
    }
    vec.retain(|t| t.key != key);
    let mut file = File::create("tags/en.json").unwrap();
    file.write_all(serde_json::to_string(&vec).unwrap().as_bytes())
        .unwrap();
    drop(en_tags);
    refresh_tags(lang).await;
    Ok(())
}
