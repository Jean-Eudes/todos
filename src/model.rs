use futures::Stream;
use std::error::Error;
use strum_macros::{Display, EnumString};

#[cfg(test)]
use mockall::automock;
use sqlx::{Postgres, Transaction};

#[derive(Clone)]
pub struct User {
    pub id: i32,
    pub login: String,
    pub password: String,
}

impl User {
    pub fn new(id: i32, login: String, password: String) -> User {
        User {
            id,
            login,
            password,
        }
    }
}

pub struct Todo {
    id: i32,
    title: String,
    status: Status,
}

impl Todo {
    pub fn new(id: i32, title: String, status: Status) -> Todo {
        Todo { id, title, status }
    }

    pub fn cancel(&mut self) -> bool {
        if self.status != Status::Pending {
            return false;
        }
        self.status = Status::Cancelled;
        true
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn status(&self) -> &Status {
        &self.status
    }
}

#[cfg_attr(test, automock)]
pub trait TodoPort {
    async fn load_by_id(&self, id: i32) -> Option<Todo>;
    async fn insert_new_todo<'a, 'b>(
        &self,
        transaction: &'a mut Transaction<'b, Postgres>,
        title: String,
        user_id: i32,
    ) -> Result<Todo, Box<dyn Error>>;

    async fn cancel(&self, id: i32) -> Result<(), String>;

    async fn load_stream(&self) -> impl Stream<Item = Result<Todo, String>>;

    async fn load(&self) -> Result<Vec<Todo>, Box<dyn Error>>;
}

#[derive(Display, EnumString, PartialEq)]
pub enum Status {
    Active,
    Pending,
    Cancelled,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_cancel_a_pending_task() {
        // Given
        let mut pending_task = Todo::new(1, "task 1".to_string(), Status::Pending);

        // When
        pending_task.cancel();

        // Then
        assert!(matches!(pending_task.status, Status::Cancelled));
    }
    #[test]
    fn should_not_cancel_a_completing_task() {
        // Given
        let mut pending_task = Todo::new(1, "task 1".to_string(), Status::Active);

        // When
        pending_task.cancel();

        // Then
        assert!(matches!(pending_task.status, Status::Active));
    }
}
