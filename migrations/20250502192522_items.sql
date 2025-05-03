-- Add migration script here
create table if not exists items (
    id integer primary key autoincrement,
    name text not null,
    description text not null,
    originator integer references users(id)
);

-- Add some sample data
insert into items (name, description, originator)
values (
   'test item user1.1',
   'test description user1.1',
    (select id from users where username = 'user1')
);

insert into items (name, description, originator)
values (
   'test item user1.2',
   'test description user1.2',
   (select id from users where username = 'user1')
);

insert into items (name, description, originator)
values (
   'test item user2.1',
   'test description user2.1',
   (select id from users where username = 'user2')
);

insert into items (name, description, originator)
values (
   'test item user2.2',
   'test description user2.2',
   (select id from users where username = 'user2')
);

insert into items (name, description, originator)
values (
   'test item admin.1',
   'test description admin.1',
   (select id from users where username = 'admin')
);
