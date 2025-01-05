use crate::model::{Todo, TodoPort};
use crate::usecase::TodoError::{AlreadyCancel, DatabaseError, NotFound};
use futures::TryFutureExt;
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
        write!(f, "{}", self)
    }
}

pub async fn cancel_todo(port: &impl TodoPort, id: i32, _user_id: i32) -> Result<(), TodoError> {
    if let Some(mut todo) = port.load_by_id(id).await {
        if !todo.cancel() {
            return Err(AlreadyCancel);
        }
        port.cancel(todo.id()).map_err(|_err| DatabaseError).await
    } else {
        Err(NotFound)
    }
}

pub async fn create_todo(
    port: &impl TodoPort,
    title: String,
    user_id: i32,
) -> Result<Todo, Box<dyn Error>> {
    port.insert_new_todo(title, user_id).await
}

#[cfg(test)]
mod tests {
    use mockall::predicate;
    use crate::model::{MockTodoPort, Status};
    use super::*;

    #[tokio::test]
    async fn should_cancel_when_state_exists_and_is_pending() {
        let mut mock = MockTodoPort::new();
        mock.expect_load_by_id()
            .with(predicate::eq(1))
            .returning(|id| Some(Todo::new(id, "".to_string(), Status::Pending)));

        mock.expect_cancel()
            .with(predicate::eq(1))
            .returning(|_id| Ok(()));

        let todo = cancel_todo(&mock, 1, 1).await;
        dbg!(&todo);
        assert_eq!(Ok(()), todo)
    }

    #[tokio::test]
    async fn should_return_not_found_where_todo_not_exist() {
        let mut mock = MockTodoPort::new();
        mock.expect_load_by_id()
            .with(predicate::eq(1))
            .returning(|_id| None);

        let todo = cancel_todo(&mock, 1, 1).await;
        assert_eq!(Err(NotFound), todo)
    }

    #[tokio::test]
    async fn should_return_already_cancel_where_todo_is_cancel() {
        let mut mock = MockTodoPort::new();
        mock.expect_load_by_id()
            .with(predicate::eq(1))
            .returning(|id| Some(Todo::new(id, "".to_string(), Status::Cancelled)));

        let todo = cancel_todo(&mock, 1, 1).await;
        assert_eq!(Err(AlreadyCancel), todo)
    }

}