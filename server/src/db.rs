use crate::blog::BlogPost;
use crate::submission::Submission;

use std::convert::TryInto;
use std::env;
use std::error::Error;
use std::sync::Arc;

use mongodb::bson;
use mongodb::bson::doc;
use mongodb::{
    options::{ClientOptions, UpdateModifications, UpdateOptions},
    Client, Collection,
};

use tokio::stream::StreamExt;

pub type DBState = Arc<DB>;

pub struct DB {
    blog_collection: Collection,
    contact_collection: Collection,
}

impl DB {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let config_options = ClientOptions::parse(
            &env::var("MONGO_URL").expect("Invalid/nonexistent MONGO_URL entry."),
        )
        .await?;
        let client = Client::with_options(config_options)?;

        Ok(Self {
            blog_collection: client.database("blog").collection("blog_pages"),
            contact_collection: client.database("contact").collection("submissions"),
        })
    }

    pub async fn get_num_of_contacts(&self) -> Result<usize, Box<dyn Error>> {
        Ok(self.contact_collection
            .count_documents(None, None)
            .await?
            .try_into()?)
    }

    pub async fn insert_submission(&self, submission: Submission) -> Result<(), Box<dyn Error>> {
        self.contact_collection.update_one(doc!{}, UpdateModifications::Document(bson::to_document(&submission)?), None).await?;
        Ok(())
    }

    pub async fn get_recent_submissions(&self, start: u32, end: u32) -> Result<Vec<Submission>, Box<dyn Error>> {
        let submissions: Vec<Submission> = self.contact_collection
            .aggregate(
                vec![
                    doc! {
                        "$sort": { "date": -1 }
                    },
                    doc! {
                        "$limit": start + end,
                    },
                    doc! {
                        "$skip": start
                    },
                ],
                None,
            )
            .await?
            .map(|document| {
                bson::from_document(document.map_err(|e| Box::new(e) as Box<dyn Error>)?)
                    .map_err(|e| Box::new(e) as Box<dyn Error>)
            })
            .collect::<Result<Vec<Submission>, Box<dyn Error>>>()
            .await?;
        Ok(submissions)
    }

    pub async fn get_num_of_blogs(&self) -> Result<usize, Box<dyn Error>> {
        Ok(self.blog_collection
            .count_documents(None, None)
            .await?
            .try_into()?)
    }

    pub async fn upsert_blog(&self, blog: &BlogPost) -> Result<(), Box<dyn Error>> {
        self.blog_collection
            .update_one(
                doc! { "url": &blog.url },
                UpdateModifications::Document(bson::to_document(blog)?),
                UpdateOptions::builder().upsert(true).build(),
            )
            .await?;
        Ok(())
    }

    pub async fn delete_blog<'a>(&self, url: &'a str) -> Result<(), Box<dyn Error>> {
        self.blog_collection
            .delete_one(doc! { "url": url }, None)
            .await?;
        Ok(())
    }

    pub async fn get_blog<'a>(&self, blog_url: &'a str) -> Result<BlogPost, Box<dyn Error>> {
        let blog: BlogPost = bson::from_document(
            self.blog_collection
                .find_one(doc! { "url": &blog_url }, None)
                .await?
                .ok_or("Could not find blog post")?,
        )?;

        Ok(blog)
    }

    pub async fn get_recent_blogs<'a>(
        &self,
        start: u32,
        end: u32,
    ) -> Result<Vec<BlogPost>, Box<dyn Error>> {
        let blogs: Vec<BlogPost> = self.blog_collection
            .aggregate(
                vec![
                    doc! {
                        "$sort": { "date": -1 }
                    },
                    doc! {
                        "$limit": start + end,
                    },
                    doc! {
                        "$skip": start
                    },
                ],
                None,
            )
            .await?
            .map(|document| {
                bson::from_document(document.map_err(|e| Box::new(e) as Box<dyn Error>)?)
                    .map_err(|e| Box::new(e) as Box<dyn Error>)
            })
            .collect::<Result<Vec<BlogPost>, Box<dyn Error>>>()
            .await?;
        Ok(blogs)
    }
}
