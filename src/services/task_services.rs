use core::task;

use super::super::DbConn;
use crate::{auth::AuthenticatedUser, models::{NewTask, NewTaskHabit, TaskUpdate, TaskWithHabit}};
use crate::schema::{tasks, habits, task_habit};
use diesel::ExpressionMethods;
use diesel::prelude::*;
use rocket::serde::json::Json;
//use serde_json::json;
use crate::models::Task;
use crate::schema::tasks::dsl::id;



pub struct TaskService {
    db: DbConn,
}


impl TaskService {
    pub fn new(db: DbConn) -> Self {
        TaskService { db }
    }
    //TODO this filters by id rather than presenting all.
    pub async fn get_tasks(&self, auth: &AuthenticatedUser) -> Result<Json<Vec<TaskWithHabit>>, TaskUpdateError> {
        let user_id = auth.user_id;
        let tasks = self.db.run(move |c| {
            tasks::table
            .inner_join(task_habit::table.on(task_habit::task_id.eq(tasks::id)))
            .inner_join(habits::table.on(habits::id.eq(task_habit::habit_id)))
            .filter(habits::user_id.eq(user_id))
            .select((
                tasks::id,
                tasks::name,
                tasks::created_at,
                tasks::completed_at,
                tasks::is_completed,
                tasks::complexity,
                habits::name,
            ))
            .limit(1000)
            .load::<TaskWithHabit>(c)
            .map(|tasks| rocket::serde::json::Json(tasks))
        }).await;

        match tasks {
            Ok(tasks) => Ok(tasks),
            Err(_) => Err(TaskUpdateError::DatabaseError),
        }
    }

    pub async fn view_task(&self, user_id: i32, task_id: i32, auth: &AuthenticatedUser) -> Result<Json<TaskWithHabit>, TaskUpdateError> {
        if auth.user_id != user_id && auth.role != 1 {
            return Err(TaskUpdateError::AuthorizationError);
        }
        let user_id = auth.user_id;

        let task = self.db.run(move |c| {
            tasks::table
            .inner_join(task_habit::table.on(task_habit::task_id.eq(tasks::id)))
            .inner_join(habits::table.on(habits::id.eq(task_habit::habit_id)))
            .filter(habits::user_id.eq(user_id))
            .filter(tasks::id.eq(task_id))
            .select((
                tasks::id,
                tasks::name,
                tasks::created_at,
                tasks::completed_at,
                tasks::is_completed,
                tasks::complexity,
                habits::name,
            ))
            .first::<TaskWithHabit>(c)
            .map(|task| rocket::serde::json::Json(task))
        }).await;

        match task {
            Ok(task) => Ok(task),
            Err(_) => Err(TaskUpdateError::DatabaseError),
        }
    }
    //TODO this needs an associated habit_id & the habittask table updating.
    pub async fn create_task(&self, new_task: Json<NewTask>, auth: &AuthenticatedUser) -> Result<(), TaskUpdateError> {
        
        let new_task = new_task.into_inner();
        let new_task_record = NewTask {
            name: new_task.name,
            complexity: new_task.complexity,
            habit_id: new_task.habit_id,
        };  
        // IM IN THE MIDDLE OF BUILDING THIS NEW TASK RECORD, give it the fields it needs to be a task in the table and have a habit id for later.
        //then it can be associated in the task_habit_record.
        //MAYBE THE JSON PASSED IN SHOULD JUST BE A TASK BUT THEN ADD SKIP DESERIALIZE JUST LIKE USER FIELD
        // CHECK WHAT I DID FOR MAKING A NEW USER IN THE USER SERVICE


        let result = self.db.run(move |c| {
            c.transaction::<_, diesel::result::Error, _>(|| {
                diesel::insert_into(tasks::table)
                    .values(&new_task_record)
                    .execute(c)?;

                let task_id = tasks::table
                .order(id.desc())
                .select(id)
                .first::<i32>(c)?;

                let new_task_habit_record = NewTaskHabit {
                    task_id,
                    habit_id: new_task.habit_id,
                    contribution: new_task.contribution,
                };

                
    
            })
        }).await;
        let user_id = auth.user_id;
        let result = self.db.run(move |c| {
            diesel::insert_into(tasks::table)
                .values(&*new_task)
                .execute(c)
        }).await;

        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(TaskUpdateError::DatabaseError),
        }
    }

}

pub enum TaskUpdateError {
    AuthorizationError,
    DatabaseError,
    NoTaskFound,
}
