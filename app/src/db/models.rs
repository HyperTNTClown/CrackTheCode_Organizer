
use diesel::prelude::*;
use hsh::models::hash::Hash;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::db::schema::users)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
    pub salt: String,
    pub admin: bool,
}

#[derive(Insertable)]
#[diesel(table_name = crate::db::schema::users)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub password: String,
    pub salt: String,
}

impl NewUser {
    pub fn new(name: &str, password: &str, email: &str) -> Self {
        let salt = Hash::generate_salt("argon2i").unwrap();
        let hash = Hash::generate_hash(password, &salt, "argon2i").unwrap();

        Self {
            name: name.to_string(),
            email: email.to_string(),
            password: hex::encode(hash),
            salt,
        }
    }
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::puzzles)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Puzzle {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub created: chrono::NaiveDateTime,
    pub modified: chrono::NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::db::schema::puzzles)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct NewPuzzle {
    pub name: String,
    pub description: String,
}

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::db::schema::puzzle_set)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct PuzzleSet {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub created: chrono::NaiveDateTime,
    pub modified: chrono::NaiveDateTime,
}

impl PuzzleSet {
    pub fn fetch_puzzles(&self, conn: &mut MysqlConnection) -> Vec<Puzzle> {
        use crate::db::schema::puzzle_set_refs::dsl::{puzzle_set_id, puzzle_set_refs};
        use crate::db::schema::puzzles::dsl::{id, puzzles};

        let puzzle_refs = puzzle_set_refs
            .filter(puzzle_set_id.eq(self.id))
            .load::<PuzzleSetRef>(conn)
            .unwrap();
        let mut puzzle_vec = Vec::new();

        for puzzle_ref in puzzle_refs {
            let puzzle = puzzles
                .filter(id.eq(puzzle_ref.puzzle_id))
                .first::<Puzzle>(conn)
                .unwrap();
            puzzle_vec.push(puzzle);
        }

        puzzle_vec
    }
}

#[derive(Insertable)]
#[diesel(table_name = crate::db::schema::puzzle_set)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct NewPuzzleSet {
    pub name: String,
    pub description: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::puzzle_set_refs)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct PuzzleSetRef {
    pub id: i32,
    pub puzzle_set_id: i32,
    pub puzzle_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = crate::db::schema::puzzle_set_refs)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct NewPuzzleSetRef {
    pub puzzle_set_id: i32,
    pub puzzle_id: i32,
}
