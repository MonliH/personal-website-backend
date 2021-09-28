use actix_web::{get, web, HttpResponse};
use serde::{Deserialize, Serialize};

use std::borrow::Cow;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EmbedQuery {
    color: Option<String>,
    text: Option<String>,
}

#[get("/discord-embed")]
pub async fn simple_embed(web::Query(q): web::Query<EmbedQuery>) -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body(format!(
        r#"<!DOCTYPE html>
<html prefix="og: https://ogp.me/ns#">
  <head>
    <title>Jonathan Li's Embeds</title>
    <meta content="" property="og:title" />
    <meta content="" property="og:description" />
    <meta content="https://api.jonat.li/magic.png" property="og:image" />
    <meta name="theme-color" content="\#{}" />
    <link
      type="application/json+oembed"
      href="https://api.jonat.li/discord-embed/json?author={}"
    />
  </head>
  <body>
      <h1>Hello</h1>
  </body>
</html>"#,
        q.color
            .map(|val| Cow::Owned(val))
            .unwrap_or(Cow::Borrowed("fc5549")),
        q.text
            .map(|val| Cow::Owned(val))
            .unwrap_or(Cow::Borrowed("Nothing To See Here..."))
    ))
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct JsonDiscordEmbed {
    author: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct JsonDiscordEmbedResponse {
    author_name: String,
    #[serde(rename = "type")]
    _type: &'static str,
}

#[get("/discord-embed/json")]
pub async fn discord_embed_json(web::Query(q): web::Query<JsonDiscordEmbed>) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/json")
        .body(
            serde_json::to_string(&JsonDiscordEmbedResponse {
                author_name: q.author,
                _type: "photo",
            })
            .unwrap(),
        )
        .into()
}
