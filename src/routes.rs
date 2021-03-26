//#[macro_use]
//extern crate rocket_contrib;
// use rocket::request::Form;
// use rocket::request::FromFormValue;
use rocket_contrib::json::Json;

extern crate diesel;

extern crate vaulty;
use self::vaulty::*;

// use self::diesel::prelude::*;
use self::models::*;
use crate::db_service::*;

#[get("/<path_context>/<path_key>", format = "application/json")]
pub fn get_entity(path_context: Option<String>, path_key: Option<String>) -> Json<ApiEntity> {
    let connection = establish_connection();
    let db_value = search_entity_by_key(connection, path_context.unwrap(), path_key.unwrap());

    let found_entity = match db_value {
        Some(val) => val,
        None => {
            return Json(ApiEntity {
                context: None,
                key: None,
                value: None,
            });
        }
    };

    return Json(ApiEntity {
        context: found_entity.context,
        key: found_entity.key,
        value: found_entity.value, //     Entity {
                                   //     context: found_entity.context,
                                   //     key: found_entity.key,
                                   //     value: found_entity.value,
                                   // }
    });
}

#[post(
    "/<context>/<path_key>",
    format = "application/json",
    data = "<entity>"
)]
pub fn create_entity(context: String, path_key: String, entity: Json<ApiEntity>) {
    let connection = establish_connection();
    create_entity_in_db(
        connection,
        context.to_string(),
        path_key.to_string(),
        entity.value.as_ref().unwrap().to_string(),
    );
    // let new_card = AssignedFuelCard {
    //     active: true,
    //     id: 1,
    //     created_timestamp: Some(Utc::now().naive_utc()),
    //     pan: fuel_card.fuelCardPan,
    //     provider: fuel_card.provider,
    //     provider_id: None,
    //     updated_timestamp: Some(Utc::now().naive_utc()),
    //     user_id: fuel_card.userId,
    //     vin: fuel_card.vin,
    //     fuelling_card: Some(1),
    //     fuelling_user: Some(1),
    // };

    // println!(
    //     "card is {}",
    //     fuelCard.into_inner().userId.unwrap_or("".to_string())
    // );
}

/*
FuelCardTO
    String provider;
    String fuelCardPan;
    String userId;
    String vin;
    String error;

DeletedCardTO
    private String vin;
    private String userId;
    private CompanyClient provider;
    private UUID providerId;
    private String error;




-------
POST /fuelCard
-> FuelCardTO
<- FuelCardTO

GET /fuelCard
-> vin (optional)
-> userId (optional)
-> provider
<- FuelCardTO

DELETE /fuelCard
-> vin (optional)
-> userId (optional)
-> provider
<- FuelCardTO



    */
