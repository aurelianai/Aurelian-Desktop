CREATE TABLE chats (
    id INTEGER PRIMARY KEY NOT NULL,
    title TEXT NOT NULL
);

CREATE TABLE messages (
    id INTEGER PRIMARY KEY NOT NULL,
    role TEXT CHECK( role IN ('USER', 'MODEL') ),
    content TEXT NOT NULL,
    chat_id INTEGER NOT NULL,
    FOREIGN KEY(chat_id) REFERENCES chats(id)
);
