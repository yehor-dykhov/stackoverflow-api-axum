use async_trait::async_trait;
use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::models::{DBError, Question, QuestionDetail};

#[async_trait]
pub trait QuestionsDao {
    async fn create_question(&self, question: Question) -> Result<QuestionDetail, DBError>;
    async fn delete_question(&self, question_uuid: String) -> Result<(), DBError>;
    async fn get_questions(&self) -> Result<Vec<QuestionDetail>, DBError>;
}

pub struct QuestionsDaoImpl {
    db: PgPool,
}

impl QuestionsDaoImpl {
    pub fn new(db: PgPool) -> QuestionsDaoImpl {
        QuestionsDaoImpl { db }
    }
}

#[async_trait]
impl QuestionsDao for QuestionsDaoImpl {
    async fn create_question(&self, question: Question) -> Result<QuestionDetail, DBError> {
        let record = sqlx::query(
            "INSERT INTO questions ( title, description ) VALUES ( $1, $2 ) RETURNING *",
        )
        .bind(question.title)
        .bind(question.description)
        .fetch_one(&self.db)
        .await
        .map_err(|e| DBError::Other(Box::new(e)))?;

        Ok(QuestionDetail {
            question_uuid: record
                .try_get::<sqlx::types::Uuid, usize>(0)
                .unwrap()
                .to_string(),
            title: record.try_get::<String, usize>(1).unwrap(),
            description: record.try_get::<String, usize>(2).unwrap(),
            created_at: record
                .try_get::<time::PrimitiveDateTime, usize>(3)
                .unwrap()
                .to_string(),
        })
    }

    async fn delete_question(&self, question_uuid: String) -> Result<(), DBError> {
        let uuid = Uuid::parse_str(question_uuid.as_str())
            .map_err(|e| DBError::InvalidUUID(e.to_string()))?;

        sqlx::query("DELETE FROM questions WHERE question_uuid = $1")
            .bind(uuid)
            .execute(&self.db)
            .await
            .map_err(|e| DBError::Other(Box::new(e)))?;

        Ok(())
    }

    async fn get_questions(&self) -> Result<Vec<QuestionDetail>, DBError> {
        let row_records = sqlx::query("SELECT * FROM questions")
            .fetch_all(&self.db)
            .await
            .map_err(|e| DBError::Other(Box::new(e)))?;

        let questions: Vec<QuestionDetail> = row_records
            .into_iter()
            .map(|row| QuestionDetail {
                question_uuid: row
                    .try_get::<sqlx::types::Uuid, usize>(0)
                    .unwrap()
                    .to_string(),
                title: row.try_get::<String, usize>(1).unwrap(),
                description: row.try_get::<String, usize>(2).unwrap(),
                created_at: row
                    .try_get::<time::PrimitiveDateTime, usize>(3)
                    .unwrap()
                    .to_string(),
            })
            .collect();

        Ok(questions)
    }
}
