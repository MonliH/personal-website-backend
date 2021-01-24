use crate::blog::{BlogPostHTML, BlogPostPreview};
use crate::submission::SubmissionDate;

use std::convert::TryInto;
use std::env;
use std::error::Error;
use std::sync::Arc;

use mongodb::bson::{self, doc, document::Document, oid::ObjectId};
use mongodb::{
    options::{FindOneOptions, FindOptions, UpdateModifications, UpdateOptions},
    Client, Collection,
};

use serde::de::DeserializeOwned;

use tokio::stream::StreamExt;

pub type DBState = Arc<DB>;

pub struct DB {
    blog_collection: Collection,
    contact_collection: Collection,
}

impl DB {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let client =
            Client::with_uri_str(&env::var("MONGO_URL").expect("Non-existent MONGO_URL entry."))
                .await?;

        Ok(Self {
            blog_collection: client.database("blog").collection("blog_pages"),
            contact_collection: client.database("contact").collection("submissions"),
        })
    }

    pub async fn get_single_contact(&self, id: &str) -> Result<SubmissionDate, Box<dyn Error>> {
        let blog = bson::from_document(
            self.contact_collection
                .find_one(doc! { "_id": ObjectId::with_string(id)? }, None)
                .await?
                .ok_or("Could not find submission")?,
        )?;

        Ok(blog)
    }

    pub async fn get_num_of_contacts(&self) -> Result<usize, Box<dyn Error>> {
        Ok(self
            .contact_collection
            .count_documents(None, None)
            .await?
            .try_into()?)
    }

    pub async fn insert_contact(&self, submission: SubmissionDate) -> Result<(), Box<dyn Error>> {
        self.contact_collection
            .insert_one(bson::to_document(&submission)?, None)
            .await?;
        Ok(())
    }

    pub async fn delete_contact<'a>(&self, _id: &'a str) -> Result<(), Box<dyn Error>> {
        self.contact_collection
            .delete_one(doc! { "_id": ObjectId::with_string(_id)? }, None)
            .await?;
        Ok(())
    }

    pub async fn get_recent_submissions(
        &self,
        start: u32,
        end: u32,
    ) -> Result<Vec<SubmissionDate>, Box<dyn Error>> {
        let submissions: Vec<SubmissionDate> = self
            .contact_collection
            .aggregate(
                vec![
                    doc! {
                        "$sort": { "_id": -1 }
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
            .collect::<Result<Vec<SubmissionDate>, Box<dyn Error>>>()
            .await?;
        Ok(submissions)
    }

    pub async fn get_num_of_blogs(&self) -> Result<usize, Box<dyn Error>> {
        Ok(self
            .blog_collection
            .count_documents(None, None)
            .await?
            .try_into()?)
    }

    pub async fn upsert_blog<'a>(&self, blog: BlogPostHTML<'a>) -> Result<(), Box<dyn Error>> {
        self.blog_collection
            .update_one(
                doc! { "url": &blog.url },
                UpdateModifications::Document(bson::to_document(&blog)?),
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

    pub async fn get_blog<'a, T>(
        &self,
        blog_url: &'a str,
        exclude: Option<&[&'a str]>,
    ) -> Result<T, Box<dyn Error>>
    where
        T: DeserializeOwned,
    {
        let blog: T = bson::from_document(
            self.blog_collection
                .find_one(
                    doc! { "url": &blog_url },
                    FindOneOptions::builder()
                        .projection(exclude.map(|keys| {
                            keys.iter().fold(Document::new(), |mut doc, key| {
                                doc.insert(*key, 0);
                                doc
                            })
                        }))
                        .build(),
                )
                .await?
                .ok_or("Could not find blog post")?,
        )?;

        Ok(blog)
    }

    pub async fn get_all_urls(&self) -> Result<Vec<String>, Box<dyn Error>> {
        let blog_urls: Vec<String> = self
            .blog_collection
            .find(
                None,
                Some(
                    FindOptions::builder()
                        .projection(doc! {"_id": 0, "url": 1})
                        .build(),
                ),
            )
            .await?
            .map(|document| {
                Ok(document
                    .map_err(|e| Box::new(e) as Box<dyn Error>)?
                    .get_str("url")
                    .unwrap()
                    .to_string())
            })
            .collect::<Result<Vec<String>, Box<dyn Error>>>()
            .await?;
        Ok(blog_urls)
    }

    pub async fn get_recent_blogs(
        &self,
        start: u32,
        end: u32,
    ) -> Result<Vec<BlogPostPreview>, Box<dyn Error>> {
        let blogs: Vec<BlogPostPreview> = self
            .blog_collection
            .aggregate(
                vec![
                    doc! { "$sort": { "date": -1 } },
                    doc! { "$limit": start + end, },
                    doc! { "$skip": start },
                    doc! { "$unset": ["html_contents", "md_contents"] },
                ],
                None,
            )
            .await?
            .map(|document| {
                bson::from_document(document.map_err(|e| Box::new(e) as Box<dyn Error>)?)
                    .map_err(|e| Box::new(e) as Box<dyn Error>)
            })
            .collect::<Result<Vec<BlogPostPreview>, Box<dyn Error>>>()
            .await?;
        Ok(blogs)
    }
}
