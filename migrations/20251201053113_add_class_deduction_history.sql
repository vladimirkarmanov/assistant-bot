create table class_deduction_history (
    class_deduction_history_id integer primary key autoincrement,
    created_at text not null default current_timestamp,
    class_id integer not null,
    user_id integer not null,
    foreign key (class_id) references class(class_id) on delete cascade,
    foreign key (user_id) references user(user_id) on delete cascade
);
