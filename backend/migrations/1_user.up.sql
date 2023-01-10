CREATE TABLE "user" (
    id UUID PRIMARY KEY,
    sid UUID NOT NULL,
    username TEXT UNIQUE NOT NULL,
    hashed_password TEXT NOT NULL,
    joined_at TIMESTAMP WITH TIME ZONE NOT NULL
);
