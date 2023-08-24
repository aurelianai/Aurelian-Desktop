// @generated automatically by Diesel CLI.

diesel::table! {
    chats (id) {
        id -> Integer,
        title -> Text,
    }
}

diesel::table! {
    messages (id) {
        id -> Integer,
        role -> Text,
        content -> Text,
        chat_id -> Integer,
    }
}

diesel::joinable!(messages -> chats (chat_id));

diesel::allow_tables_to_appear_in_same_query!(
    chats,
    messages,
);
