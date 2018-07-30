table! {
    albums (id) {
        id -> Text,
        name -> Text,
        description -> Nullable<Text>,
        cover_image -> Nullable<Text>,
        username -> Nullable<Text>,
        created_date -> Timestamp,
        last_modified_date -> Timestamp,
        deleted_date -> Nullable<Timestamp>,
    }
}

table! {
    pictures (id) {
        id -> Text,
        name -> Text,
        description -> Nullable<Text>,
        score -> Int4,
        file -> Text,
        album_id -> Nullable<Text>,
        created_date -> Timestamp,
        last_modified_date -> Timestamp,
        deleted_date -> Nullable<Timestamp>,
    }
}

table! {
    users (username) {
        username -> Text,
        display_name -> Text,
        bio -> Nullable<Text>,
        display_image -> Text,
        password -> Text,
        created_date -> Timestamp,
        last_modified_date -> Timestamp,
        deleted_date -> Nullable<Timestamp>,
    }
}

joinable!(albums -> users (username));
joinable!(pictures -> albums (album_id));

allow_tables_to_appear_in_same_query!(
    albums,
    pictures,
    users,
);
