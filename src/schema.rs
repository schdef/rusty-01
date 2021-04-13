table! {
    vaultstore (id) {
        id -> Int4,
        created_timestamp -> Nullable<Timestamp>,
        context -> Nullable<Varchar>,
        entity_key -> Nullable<Varchar>,
        entity_value -> Nullable<Varchar>,
    }
}
