/************************************************************************
*                        USERS Table
************************************************************************/

CREATE TABLE IF NOT EXISTS USERS (
    ID              serial          PRIMARY KEY,
    USER_ID         varchar(50)     UNIQUE NOT NULL,
    USERNAME        varchar(250)    UNIQUE NOT NULL,
    FIRST_NAME      varchar(250)    NOT NULL,
    LAST_NAME       varchar(250)    NOT NULL,
    FUNCTION        varchar(50)     NOT NULL,
    EMAIL           varchar(250)    NOT NULL,
    ATTRIBUTES      jsonb,
    CREATED_AT      timestamptz     NOT NULL,
    CREATED_BY      varchar(250)    NOT NULL,
    UPDATED_AT      timestamptz,
    UPDATED_BY      varchar(250),
    VERSION         integer         DEFAULT 1 CHECK(version > 0)
);

DROP INDEX IF EXISTS USERS_USER_ID_IDX;
DROP INDEX IF EXISTS USERS_USERNAME_IDX;
DROP INDEX IF EXISTS USERS_FUNCTION_IDX;

CREATE UNIQUE INDEX USERS_USER_ID_IDX ON USERS(USER_ID);
CREATE UNIQUE INDEX USERS_USERNAME_IDX ON USERS(USERNAME);