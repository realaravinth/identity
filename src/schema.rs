table! {
    users (user_id) {
        user_id -> Int4,
        username -> Varchar,
        user_uuid -> Uuid,
        hash -> Bytea,
        salt -> Varchar,
        email -> Nullable<Varchar>,
        role -> Varchar,
        name -> Varchar,
        created_at -> Timestamp,
    }
}
