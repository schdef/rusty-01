extern crate diesel;
extern crate vaulty;
use self::vaulty::*;

use self::diesel::prelude::*;
use self::models::*;
use self::schema::vaultstore::dsl::*;

use self::schema::*;
use chrono::Utc;

// use rocket_contrib::json::Json;

pub fn search_entity_by_key(
    connection: PgConnection,
    context_of_key: String,
    search_for_key: String,
) -> Option<Entity> {
    // let results: Vec<AssignedFuelCard> = Vec::new();
    // return results;
    println!("context={}, key={}", context_of_key, search_for_key);
    let result = vaultstore
        .filter(context.eq(context_of_key))
        .first::<Entity>(&connection)
        .optional()
        .expect("error loading");
    // println!("result={}", result);
    return result;
    // let results = vaultstore
    //     .filter(context.eq(context_of_key))
    //         // .first::<AssignedFuelCard>(&connection)
    // .optional()
    // .expect("Error loading posts");

    // .filter(vin.eq(search_for_vin))
    // .filter(active.eq(true))
    // .filter(provider.eq(search_for_provider))
    // .filter(user_id.is_null())
    // .first::<AssignedFuelCard>(&connection)
    // .optional()
    // .expect("Error loading posts");
    // println!("Displaying {} cards", results.len());
    // return None;
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
    // let new_card = (
    //     id.eq(1),
    //     active.eq(true),
    //     created_timestamp.eq(Some(Utc::now().naive_utc())),
    //     pan.eq(&fuel_card.fuelCardPan),
    //     provider.eq(&fuel_card.provider),
    //     updated_timestamp.eq(Some(Utc::now().naive_utc())),
    //     user_id.eq(&fuel_card.userId),
    //     // vin.eg(Some(fuel_card.vin)),
    //     // fuelling_card.eg(Some(1)),
    //     // fuelling_user.eg(Some(1)),
    // );
    // diesel::insert_into(assigned_fuelling_card)
    //     .values(new_card)
    //     .execute(&connection);
}

/*

extern crate vaulty;
extern crate diesel;
use self::vaulty::*;

use self::diesel::prelude::*;
use self::models::*;
use self::schema::assigned_fuelling_card::dsl::*;

pub fn search_card_by_vin(connection, search_for_vin) -> Vec<AssignedFuelCard> {
    let results: Vec<AssignedFuelCard> = Vec::new();
    return results;
    // assigned_fuelling_card
    //     .filter(vin.eq(&search_for_vin))
    //     .load::<AssignedFuelCard>(&connection)
    //     .expect("Error loading posts");
}

*/
