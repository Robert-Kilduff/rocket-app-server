

use super::super::DbConn;
use crate::{auth::AuthenticatedUser, models::{NewTask, NewTaskHabit, TaskWithHabit, NewTaskRequest}};
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

    pub async fn create_task(&self, new_task: Json<NewTaskRequest>, _auth: &AuthenticatedUser) -> Result<(), TaskUpdateError> {
            
        let new_task = new_task.into_inner();
    
        let new_task_record = NewTask {
            name: new_task.name,
            is_completed: new_task.is_completed.unwrap_or(false),
            complexity: new_task.complexity.unwrap_or(2),
        };
    
        let result = self.db.run(move |c| {
            c.transaction::<_, diesel::result::Error, _>(|c| {
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
    
                diesel::insert_into(task_habit::table)
                    .values(&new_task_habit_record)
                    .execute(c)?;
    
                Ok(())
            })
        }).await;
    
        result.map_err(|_| TaskUpdateError::DatabaseError).map(|_| ()) //dont like this but it stops error errors
    }

    pub async fn delete_task(&self, task_id: i32, auth: &AuthenticatedUser) -> Result<(), TaskUpdateError> {
        let task = self.db.run(move |c| {
            c.transaction::<_, diesel::result::Error, _>(|c| {
                let task = tasks::table
                .filter(tasks::id.eq(task_id))
                .first::<Task>(c)
                .optional()?;

                let taskhabit = task_habit::table
                .filter(task_habit::task_id.eq(task_id))
                .select((task_habit::task_id, task_habit::habit_id, task_habit::contribution))
                .first::<(i32, i32, Option<i32>)>(c) //tupled and mapped after because diesel doesn't like it directly
                .map(|(task_id, habit_id, contribution)| {
                    NewTaskHabit {
                        task_id,
                        habit_id,
                        contribution,
                    }
                })
                .optional()?;


                if let (Some(task), Some(taskhabit)) = (task, taskhabit) {
                    diesel::delete(tasks::table.filter(tasks::id.eq(task.id))).execute(c)?;
                    diesel::delete(task_habit::table.filter(task_habit::task_id.eq(taskhabit.task_id))).execute(c)?;
                    Ok(())
                } else {
                    Err(diesel::result::Error::NotFound)
                }
               

            })

        }).await;

        match task {
            Ok(_) => Ok(()),
            Err(diesel::result::Error::NotFound) => Err(TaskUpdateError::NoTaskFound),
            Err(_) => Err(TaskUpdateError::DatabaseError),
        }
    }
        
    pub async fn update_task(&self, task_id: i32, task: Json<NewTaskRequest>, auth: &AuthenticatedUser) -> Result<(), TaskUpdateError> {
        let updatetask = task.into_inner();

        let result = self.db.run(move |c| {
            c.transaction::<_, diesel::result::Error, _>(|c| {
                let task = tasks::table
                    .filter(tasks::id.eq(task_id))
                    .first::<Task>(c)
                    .optional()?;

                let taskhabit = task_habit::table
                    .filter(task_habit::task_id.eq(task_id))
                    .select((task_habit::task_id, task_habit::habit_id, task_habit::contribution))
                    .first::<(i32, i32, Option<i32>)>(c) //tupled and mapped after because diesel doesn't like it directly
                    .map(|(task_id, habit_id, contribution)| {
                        NewTaskHabit {
                            task_id,
                            habit_id,
                            contribution,
                        }
                    })
                    .optional()?;

                if let (Some(task), Some(taskhabit)) = (task, taskhabit) {
                    if taskhabit.habit_id != updatetask.habit_id {
                        
                        diesel::update(task_habit::table.filter(task_habit::task_id.eq(task_id)))
                            .set((
                                task_habit::habit_id.eq(updatetask.habit_id),
                                task_habit::contribution.eq(updatetask.contribution),
                            ))
                            .execute(c)?;
                        //TODO HERE Perhaps some change to the struct being passed in order to satisfy.
                        diesel::update(tasks::table.filter(tasks::id.eq(task_id)))
                            .set((tasks::name.eq(updatetask.name),
                            tasks::is_completed.eq(updatetask.is_completed),
                            tasks::complexity.eq(updatetask.complexity),
                            ))
                            .execute(c)?;
                            Ok(())

                    } else {
                        diesel::update(tasks::table.filter(tasks::id.eq(task_id)))
                            .set((tasks::name.eq(updatetask.name),
                            tasks::is_completed.eq(updatetask.is_completed),
                            tasks::complexity.eq(updatetask.complexity),
                            ))
                            .execute(c)?;
                            Ok(())


                    }
                } else {
                    Err(diesel::result::Error::NotFound)
                }   
                
            })
            }).await;

        match result {
            Ok(result) => Ok(()),
            Err(diesel::result::Error::NotFound) => Err(TaskUpdateError::NoTaskFound),
            Err(_) => Err(TaskUpdateError::DatabaseError),
        }
    
        }

    }



pub enum TaskUpdateError {
    AuthorizationError,
    DatabaseError,
    NoTaskFound,
}
