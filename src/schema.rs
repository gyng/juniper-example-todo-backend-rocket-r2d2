use diesel;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use juniper::{Context as JuniperContext, FieldResult, ResultExt};
use r2d2;
use r2d2_diesel;

use models::{NewTodo, Todo};

pub struct Context {
    pub pool: r2d2::Pool<r2d2_diesel::ConnectionManager<SqliteConnection>>,
}

impl JuniperContext for Context {}

graphql_object!(Todo: () |&self| {
    description: "some todo item"

    field id() -> i32 as "unique id" {
        self.id
    }

    field title() -> &str as "user-editable title" {
        &self.title
    }

    field completed() -> bool as "whether this todo is completed" {
        self.completed
    }
});

pub struct QueryRoot;

graphql_object!(QueryRoot: Context |&self| {
    field todoItems(&executor) -> FieldResult<Vec<Todo>>
    as "get all todo items by date"
    {
        use ::db::todos::dsl;

        let connection = executor.context().pool.clone().get().unwrap();

        dsl::todos.order(dsl::id)
            .load::<Todo>(&*connection)
            .to_field_result()
    }
});

pub struct MutationRoot;

graphql_object!(MutationRoot: Context |&self| {
    field add_todo(&executor, title: String, completed: bool) -> FieldResult<Todo>
    as "create a new todo"
    {
        use ::db::todos::dsl;

        let connection = executor.context().pool.clone().get().unwrap();

        connection.transaction(|| {
            let new_post = NewTodo {
                title: &title,
                completed: completed
            };

            diesel::insert(&new_post).into(::db::todos::table)
                .execute(&*connection)?;

            dsl::todos.order(dsl::id.desc())
                .first::<Todo>(&*connection)
        }).to_field_result()
    }

    field update_todo(&executor, id: i32, completed: Option<bool>, title: Option<String>)
    -> FieldResult<Option<Todo>> as "update existing todo"

    {
        use ::db::todos::dsl;
        let connection = executor.context().pool.clone().get().unwrap();

        let updated = jtry!(diesel::update(dsl::todos.find(id))
        .set((
            completed.map(|completed| dsl::completed.eq(completed)),
            title.map(|title| dsl::title.eq(title))
        ))
        .execute(&*connection));

        if updated == 0 {
            Ok(None)
        } else {
            Ok(Some(jtry!(dsl::todos.find(id)
                .get_result::<Todo>(&*connection))))
        }
    }

    field delete_todo(&executor, id: i32)
    -> FieldResult<Option<i32>> as "delete existing todo"

    {
        use ::db::todos::dsl;

        let connection = executor.context().pool.clone().get().unwrap();
        let deleted = jtry!(diesel::delete(dsl::todos.find(id)).execute(&*connection));

        if deleted == 1 {
            Ok(Some(id))
        } else {
            Ok(None)
        }
    }
});
