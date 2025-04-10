#![warn(clippy::cargo)]
#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]


trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;   
    #[allow(unused_variables)]
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

// trait object
struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

// trait object
struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}


#[allow(dead_code)]
pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

// rust users often give default values with Struct::default; this just points to the ::new method
impl Default for Post {
    fn default() -> Self {
        Self::new()
    }
}

impl Post {
    #[must_use]
    pub fn new() -> Self {
        Self {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }    
    
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    // #[must_use]
    // pub const fn content(&self) -> &str {
    //     ""
    // }
    /// # Panics
    ///
    /// this method doesn't panic because the methods in Post always ensure state is Some value
    #[must_use]
    pub fn content(&self) -> &str {
        #[allow(clippy::unwrap_used)]
        self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review());
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve());
        }
    }
}



#[cfg(test)]
mod tests {
    //use super::*;

    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }
}
