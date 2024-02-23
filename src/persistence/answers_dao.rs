use async_trait::async_trait;
use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::models::{postgres_error_codes, Answer, AnswerDetail, DBError};

#[async_trait]
pub trait AnswersDao {
    async fn create_answer(&self, answer: Answer) -> Result<AnswerDetail, DBError>;
    async fn delete_answer(&self, answer_uuid: String) -> Result<(), DBError>;
    async fn get_answers(&self, question_uuid: String) -> Result<Vec<AnswerDetail>, DBError>;
}

pub struct AnswersDaoImpl {
    db: PgPool,
}

impl AnswersDaoImpl {
    pub fn new(db: PgPool) -> Self {
        AnswersDaoImpl { db }
    }
}

#[async_trait]
impl AnswersDao for AnswersDaoImpl {
    async fn create_answer(&self, answer: Answer) -> Result<AnswerDetail, DBError> {
        let uuid = Uuid::parse_str(answer.question_uuid.as_str())
            .map_err(|e| DBError::InvalidUUID(e.to_string()))?;

        let record = sqlx::query(
            "INSERT INTO answers ( question_uuid, content ) VALUES ( $1, $2 ) RETURNING *",
        )
        .bind(uuid)
        .bind(answer.content)
        .fetch_one(&self.db)
        .await
        .map_err(|e| {
            if let sqlx::Error::Database(db_err) = &e {
                if db_err.code().unwrap() == postgres_error_codes::FOREIGN_KEY_VIOLATION {
                    return DBError::InvalidUUID(db_err.to_string());
                }
            }

            DBError::Other(Box::new(e))
        })?;

        Ok(AnswerDetail {
            answer_uuid: record
                .try_get::<sqlx::types::Uuid, usize>(0)
                .unwrap()
                .to_string(),
            question_uuid: record
                .try_get::<sqlx::types::Uuid, usize>(1)
                .unwrap()
                .to_string(),
            content: record.try_get::<String, usize>(2).unwrap(),
            created_at: record
                .try_get::<time::PrimitiveDateTime, usize>(3)
                .unwrap()
                .to_string(),
        })
    }

    async fn delete_answer(&self, answer_uuid: String) -> Result<(), DBError> {
        let uuid = Uuid::parse_str(answer_uuid.as_str())
            .map_err(|e| DBError::InvalidUUID(e.to_string()))?;

        sqlx::query("DELETE FROM answers WHERE answer_uuid = $1")
            .bind(uuid)
            .execute(&self.db)
            .await
            .map_err(|e| DBError::Other(Box::new(e)))?;

        Ok(())
    }

    async fn get_answers(&self, question_uuid: String) -> Result<Vec<AnswerDetail>, DBError> {
        let uuid = Uuid::parse_str(question_uuid.as_str())
            .map_err(|e| DBError::InvalidUUID(e.to_string()))?;

        let row_records = sqlx::query("SELECT * FROM answers WHERE question_uuid = $1")
            .bind(uuid)
            .fetch_all(&self.db)
            .await
            .map_err(|e| DBError::Other(Box::new(e)))?;

        let answers = row_records
            .into_iter()
            .map(|row| AnswerDetail {
                answer_uuid: row
                    .try_get::<sqlx::types::Uuid, usize>(0)
                    .unwrap()
                    .to_string(),
                question_uuid: row
                    .try_get::<sqlx::types::Uuid, usize>(1)
                    .unwrap()
                    .to_string(),
                content: row.try_get::<String, usize>(2).unwrap(),
                created_at: row
                    .try_get::<time::PrimitiveDateTime, usize>(3)
                    .unwrap()
                    .to_string(),
            })
            .collect();

        Ok(answers)
    }
}
