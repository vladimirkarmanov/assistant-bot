create table daily_practice_log (
    daily_practice_log_id integer primary key autoincrement,
    minutes integer not null check (minutes >= 0),
    created_at text not null default current_timestamp,
    user_id integer not null,
    foreign key (user_id) references user(user_id) on delete cascade
);