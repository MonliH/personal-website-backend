use crate::blog::BlogPost;

use std::env;
use std::error::Error;

use mongodb::bson;
use mongodb::bson::doc;
use mongodb::{
    options::{ClientOptions, FindOptions, UpdateModifications, UpdateOptions},
    Client, Database,
};

use tokio::stream::StreamExt;

pub struct DB {
    client: Client,
    blog_db: Database,
}

impl DB {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let config_options = ClientOptions::parse(
            &env::var("MONGO_URL").expect("Invalid/nonexistent MONGO_URL entry."),
        )
        .await?;
        let client = Client::with_options(config_options)?;

        Ok(Self {
            blog_db: client.database("blog"),
            client,
        })
    }

    pub async fn upsert_blog(&mut self, blog: &BlogPost) -> Result<(), Box<dyn Error>> {
        let blog_collection = self.blog_db.collection("blog_pages");
        blog_collection
            .update_one(
                doc! { "url": &blog.url },
                UpdateModifications::Document(bson::to_document(blog)?),
                UpdateOptions::builder().upsert(true).build(),
            )
            .await?;
        Ok(())
    }

    pub async fn get_blog<'a>(&mut self, blog_url: &'a str) -> Result<BlogPost, Box<dyn Error>> {
        let blog_collection = self.blog_db.collection("blog_pages");
        let blog: BlogPost = bson::from_document(
            blog_collection
                .find_one(doc! { "url": &blog_url }, None)
                .await?
                .ok_or("Could not find blog post")?,
        )?;

        Ok(blog)
    }

    pub async fn get_recent_blogs<'a>(
        &mut self,
        start: u32,
        end: u32,
    ) -> Result<Vec<BlogPost>, Box<dyn Error>> {
        let blog_collection = self.blog_db.collection("blog_pages");
        let blog: Vec<BlogPost> = blog_collection
            .find(
                doc! {
                },
                Some(FindOptions::builder().sort(doc! { "date": 1 }).build()),
            )
            .await?
            .map(|document| {
                bson::from_document(document.map_err(|e| Box::new(e) as Box<dyn Error>)?)
                    .map_err(|e| Box::new(e) as Box<dyn Error>)
            })
            .collect::<Result<Vec<BlogPost>, Box<dyn Error>>>()
            .await?;
        Ok(blog)
    }
}
