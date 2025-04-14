use crate::model::{Todo, TodoPort};
use crate::usecase::TodoError::{AlreadyCancel, DatabaseError, NotFound};
use futures::{Stream, TryFutureExt};
use sqlx::{Pool, Postgres};
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
pub enum TodoError {
    AlreadyCancel,
    NotFound,
    DatabaseError,
}

impl Error for TodoError {}

impl Display for TodoError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AlreadyCancel => write!(f, "AlreadyCancel"),
            NotFound => write!(f, "NotFound"),
            DatabaseError => write!(f, "DatabaseError"),
        }
    }
}

pub struct TodoUseCase<T>
where
    T: TodoPort,
{
    todo_port: T,
}

impl<T> TodoUseCase<T>
where
    T: TodoPort,
{
    pub fn new(todo_port: T) -> Self {
        TodoUseCase { todo_port }
    }

    pub async fn cancel_todo(&self, id: i32, _user_id: i32) -> Result<(), TodoError> {
        if let Some(mut todo) = self.todo_port.load_by_id(id).await {
            if !todo.cancel() {
                return Err(AlreadyCancel);
            }
            self.todo_port
                .cancel(todo.id())
                .map_err(|_err| DatabaseError)
                .await
        } else {
            Err(NotFound)
        }
    }

    pub async fn create_todo(
        &self,
        pool: Pool<Postgres>,
        title: String,
        user_id: i32,
    ) -> Result<Todo, Box<dyn Error>> {
        let mut transaction = pool.clone().begin().await.unwrap();
        let todo = self
            .todo_port
            .insert_new_todo(&mut transaction, title.clone(), user_id)
            .await?;
        transaction.commit().await.unwrap();
        Ok(todo)
    }

    pub async fn load(&self) -> Result<Vec<Todo>, Box<dyn Error>> {
        let todo = self.todo_port.load().await?;
        Ok(todo)
    }
    pub async fn load_stream(&self) -> impl Stream<Item = Result<Todo, String>>  + use<'_, T> {
        self.todo_port.load_stream().await
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{MockTodoPort, Status};
    use mockall::predicate;

    #[tokio::test]
    async fn should_cancel_when_state_exists_and_is_pending() {
        // Given
        let mut todo_port = MockTodoPort::new();
        todo_port
            .expect_load_by_id()
            .with(predicate::eq(1))
            .returning(|id| Some(Todo::new(id, "".to_string(), Status::Pending)));

        todo_port
            .expect_cancel()
            .with(predicate::eq(1))
            .returning(|_id| Ok(()));

        let use_case = TodoUseCase::new(todo_port);

        // When
        let todo = use_case.cancel_todo(1, 1).await;

        // Then
        assert_eq!(Ok(()), todo)
    }

    #[tokio::test]
    async fn should_return_not_found_where_todo_not_exist() {
        // Given
        let mut todo_port = MockTodoPort::new();
        todo_port
            .expect_load_by_id()
            .with(predicate::eq(1))
            .returning(|_id| None);
        let use_case = TodoUseCase::new(todo_port);

        // When
        let todo = use_case.cancel_todo(1, 1).await;

        // Then
        assert_eq!(Err(NotFound), todo)
    }

    #[tokio::test]
    async fn should_return_already_cancel_where_todo_is_cancel() {
        // Given
        let mut todo_port = MockTodoPort::new();
        todo_port
            .expect_load_by_id()
            .with(predicate::eq(1))
            .returning(|id| Some(Todo::new(id, "".to_string(), Status::Cancelled)));

        let use_case = TodoUseCase::new(todo_port);

        // When
        let todo = use_case.cancel_todo(1, 1).await;

        // Then
        assert_eq!(Err(AlreadyCancel), todo)
    }
}
