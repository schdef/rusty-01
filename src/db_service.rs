extern crate diesel;
extern crate vaulty;
use self::vaulty::*;

use self::diesel::prelude::*;
use self::models::*;
use self::schema::vaultstore::dsl::*;

use chrono::Utc;

pub fn search_entity_by_key(
    connection: PgConnection,
    context_of_key: String,
    search_for_key: String,
) -> Option<Entity> {
    println!("context={}, key={}", context_of_key, search_for_key);
    let result = vaultstore
        .filter(context.eq(context_of_key))
        .first::<Entity>(&connection)
        .optional()
        .expect("error loading");
    // println!("result={}", result);
    return result;
}

pub fn create_entity_in_db(
    connection: PgConnection,
    context_of_key: String,
    search_for_key: String,
    value_for_key: String,
) {
    let new_entity = (
        context.eq(context_of_key),
        created_timestamp.eq(Some(Utc::now().naive_utc())),
        entity_key.eq(search_for_key),
        entity_value.eq(value_for_key),
    );
    diesel::insert_into(vaultstore)
        .values(new_entity)
        .execute(&connection);
}
