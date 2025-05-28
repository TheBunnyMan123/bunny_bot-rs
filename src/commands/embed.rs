use poise::serenity_prelude as serenity;
use crate::types::{Error, Context};
use url::Url;
use serde::{Serialize, Deserialize};
use reqwest::Client;

enum RedditPostCategory {
    SelfPost,
    Image,
    Video,
    Link,
    Unknown
}

#[derive(Serialize, Deserialize, Debug)]
struct Media {
    #[serde(rename = "type")]
    oembed: Option<OEmbed>,
    reddit_video: Option<RedditVideo>
}
#[derive(Serialize, Deserialize, Debug)]
struct OEmbed {
    provider_url: String
}
#[derive(Serialize, Deserialize, Debug)]
struct RedditVideo {
    fallback_url: Option<String>
}
#[derive(Serialize, Deserialize, Debug)]
struct Image {
    url: String
}
#[derive(Serialize, Deserialize, Debug)]
struct ImageSet {
    source: Image,
    resolutions: Vec<Image>
}
#[derive(Serialize, Deserialize, Debug)]
struct Preview {
    images: Vec<ImageSet>
}
#[derive(Serialize, Deserialize, Debug)]
struct RedditJsonDataChildData {
    subreddit: String,
    title: String,
    score: i64,
    num_comments: u32,
    permalink: String,
    selftext: String,
    author: String,
    url: String,
    #[serde(default)]
    thumbnail: String,
    #[serde(default)]
    is_video: bool,
    #[serde(default)]
    is_self: bool,
    media: Option<Media>,
    secure_media: Option<Media>,
    post_hint: Option<String>,
    preview: Option<Preview>
}
#[derive(Serialize, Deserialize, Debug)]
struct RedditJsonDataChild {
    data: RedditJsonDataChildData
}
#[derive(Serialize, Deserialize, Debug)]
struct RedditJsonData {
    children: Vec<RedditJsonDataChild>
}
#[derive(Serialize, Deserialize, Debug)]
struct RedditJson {
    data: RedditJsonData,
    king: String
}

fn get_hostname(link: &str) -> Option<String> {
    match Url::parse(link) {
        Ok(url) => url.host_str().map(|s| {s.to_string()}),
        Err(_) => None
    }
}

fn categorize_reddit_post(post_data: &RedditJsonDataChildData) -> RedditPostCategory {
    if post_data.is_self {
        return RedditPostCategory::SelfPost;
    }
    if post_data.is_video {
        return RedditPostCategory::Video;
    }

    if let Some(hint) = &post_data.post_hint {
        if hint == "image" {
            return RedditPostCategory::Image;
        }
        if hint.contains("video") {
            return RedditPostCategory::Video;
        }
        if hint == "link" {
            return RedditPostCategory::Link;
        }
    }

    let lower_url = post_data.url.to_lowercase();
    if lower_url.ends_with(".jpg")
        || lower_url.ends_with(".png")
        || lower_url.ends_with(".gif")
        || lower_url.ends_with(".jpeg")
        || lower_url.ends_with(".webp")
    {
        return RedditPostCategory::Image;
    }

    if !post_data.url.is_empty() && post_data.url != post_data.permalink {
        return RedditPostCategory::Link;
    }

    RedditPostCategory::Unknown
}

#[poise::command(
    slash_command,
    prefix_command,
    description_localized("en-US", "Embeds a URL"),
    guild_only = false
)]
pub async fn embed(ctx: Context<'_>, link: String) -> Result<(), Error> {
    match get_hostname(&link) {
        Some(hostname) => {
            match hostname.as_str() {
                "www.reddit.com" => {
                    ctx.defer().await?;

                    let json_url = match link.strip_suffix("/") {
                        Some(val) => val,
                        None => &link
                    };

                    let client = Client::new();
                    let user_agent = "TheBunnyBot (bunny-bot)";
                    let response = client
                        .get(format!("{}{}", json_url, ".json"))
                        .header(reqwest::header::USER_AGENT, user_agent)
                        .send()
                        .await?
                        .error_for_status()?;
                    
                    let json_data: serde_json::Value = response.json().await?;
                    let post_listing = &json_data[0]["data"]["children"][0];
                    let post_data: RedditJsonDataChildData = serde_json::from_value(post_listing["data"].clone())?;

                    let footer = serenity::builder::CreateEmbedFooter::new(format!("r/{} \u{2022} \u{1F44D}{} \u{2022} \u{1F4AC}{}", post_data.subreddit, post_data.score, post_data.num_comments));

                    let mut embed = serenity::builder::CreateEmbed::default()
                        .title(&post_data.title)
                        .url(&link)
                        .footer(footer)
                        .color(0xFF5700);

                    match categorize_reddit_post(&post_data) {
                        RedditPostCategory::SelfPost => {
                            embed = embed.description(&post_data.selftext)
                                .url(&format!("https://www.reddit.com{}", post_data.permalink))
                        },
                        RedditPostCategory::Image => {
                            let mut fixed_url = post_data.url.clone();
                            if fixed_url.contains("i.redd.it/") && !fixed_url.contains("/gallery/") && post_data.post_hint.as_deref() == Some("image") {
                                fixed_url = fixed_url.replacen("i.redd.it/", "i.redd.it/gallery/", 1);
                            }

                            embed = embed.url(&format!("https://www.reddit.com{}", post_data.permalink))
                                .image(fixed_url);
                        },
                        RedditPostCategory::Video => {
                            let video_url_for_embed = if let Some(media) = &post_data.secure_media {
                                if let Some(reddit_video) = &media.reddit_video {
                                    reddit_video.fallback_url.clone().unwrap_or(post_data.url)
                                } else {
                                    post_data.url.clone()
                                }
                            } else {
                                post_data.url.clone()
                            };
                            embed = embed.url(video_url_for_embed);

                            if post_data.thumbnail.starts_with("http") && post_data.thumbnail != "default" && post_data.thumbnail != "nsfw" {
                                embed = embed.thumbnail(&post_data.thumbnail);
                            }
                        },
                        RedditPostCategory::Link => {
                            embed = embed.url(&post_data.url)
                                .description(&format!("[Link to original content]({})", post_data.url));

                            if post_data.thumbnail.starts_with("http") && post_data.thumbnail != "default" && post_data.thumbnail != "nsfw" {
                                embed = embed.thumbnail(&post_data.thumbnail);
                            }
                        },
                        RedditPostCategory::Unknown => {}
                    }

                    ctx.send(poise::CreateReply::default()
                        .embed(embed)
                        .ephemeral(true)
                        .reply(true)).await?;
                }
                _ => {
                    let embed = serenity::builder::CreateEmbed::default()
                        .title("Error While Running Command")
                        .description(format!("Got unsupported url: {}", link))
                        .color(0xFF0000);

                    ctx.send(poise::CreateReply::default()
                        .embed(embed)
                        .ephemeral(true)
                        .reply(true)).await?;
                }
            }
        },
        None => {
            let embed = serenity::builder::CreateEmbed::default()
                .title("Error While Running Command")
                .description("Invalid URL given")
                .color(0xFF0000);

            ctx.send(poise::CreateReply::default()
                .embed(embed)
                .ephemeral(true)
                .reply(true)).await?;
        }
    }

    Ok(())
}

