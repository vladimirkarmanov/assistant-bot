create table class_deduction_history (
    class_deduction_history_id integer primary key autoincrement,
    created_at text not null default current_timestamp,
    class_id integer,
    foreign key (class_id) references class(class_id) on delete cascade
);