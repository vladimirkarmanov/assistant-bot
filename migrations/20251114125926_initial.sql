create table user (
    user_id integer primary key autoincrement,
    telegram_id integer not null unique,
    username text,
    created_at text not null default current_timestamp,
    updated_at text not null default current_timestamp,
    last_activity_at text not null default current_timestamp
);

create table class (
    class_id integer primary key autoincrement,
    name text not null unique,
    quantity integer not null check (quantity >= 0),
    created_at text not null default current_timestamp,
    updated_at text not null default current_timestamp,
    user_id integer,
    foreign key (user_id) references user(user_id) on delete cascade
);

create trigger trg_user_updated_at
after update on user
for each row
begin
    update user set updated_at = current_timestamp where user_id = old.user_id;
end;

create trigger trg_class_updated_at
after update on class
for each row
begin
    update class set updated_at = current_timestamp where class_id = old.class_id;
end;
