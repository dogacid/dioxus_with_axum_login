-- Create, users, groups and permission with relationships between them
-- Borrowed and modified from axum-login permissions example https://github.com/maxcountryman/axum-login/blob/main/examples/permissions/migrations/20231108213118_init.sql

create table if not exists users (
    id integer primary key autoincrement,
    username text not null unique,
    password text not null
);

create table if not exists groups (
    id integer primary key autoincrement,
    name text not null unique
);

create table if not exists permissions (
    id integer primary key autoincrement,
    name text not null unique
);

create table if not exists users_groups (
    user_id integer references users(id),
    group_id integer references groups(id),
    primary key (user_id, group_id)
);

create table if not exists groups_permissions (
    group_id integer references groups(id),
    permission_id integer references permissions(id),
    primary key (group_id, permission_id)
);


-- u: user1, p: user1234
insert into users (username, password)
values (
   'user1',
   '$argon2id$v=19$m=65536,t=3,p=1$cUp2MklxZnIxQWRmYm4xb0RhcGNDZz09$0aa+Sr7oxk8YP8Teokn7D5OBpaoTW+0S/vMen0FFUS8'
);

-- u: user2, p: user2345
insert into users (username, password)
values (
           'user2',
           '$argon2id$v=19$m=65536,t=3,p=1$cUp2MklxZnIxQWRmYm4xb0RhcGNDZz09$IjuNa9ayK8HEEEe+u8DOMyEHne60ZDBLalTTE2DtHFw'
       );

-- Insert "admin" user.
-- u: admin, p: admin1234
insert into users (username, password)
values (
   'admin',
   '$argon2id$v=19$m=65536,t=3,p=1$cUp2MklxZnIxQWRmYm4xb0RhcGNDZz09$LIo2fCffz5ahjpUqrUN5/5m/kLoWrBmMrApUhg0mSXk'
);

-- Insert "users" and "superusers" groups.
insert into groups (name) values ('users');
insert into groups (name) values ('superusers');

-- Insert individual permissions.
insert into permissions (name) values ('protected.read');
insert into permissions (name) values ('restricted.read');

-- Insert group permissions.
insert into groups_permissions (group_id, permission_id)
values (
           (select id from groups where name = 'users'),
           (select id from permissions where name = 'protected.read')
       ), (
           (select id from groups where name = 'superusers'),
           (select id from permissions where name = 'restricted.read')
       );

-- Insert users into groups.
insert into users_groups (user_id, group_id)
values (
   (select id from users where username = 'user1'),
   (select id from groups where name = 'users')
), (
   (select id from users where username = 'user2'),
   (select id from groups where name = 'users')
), (
   (select id from users where username = 'admin'),
   (select id from groups where name = 'users')
), (
   (select id from users where username = 'admin'),
   (select id from groups where name = 'superusers')
);