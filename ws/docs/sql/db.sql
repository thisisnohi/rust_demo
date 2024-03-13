drop table if exists course;

create table course(
    id serial primary key,
    teacher_id INT not null,
    name varchar(140) not null,
    time TIMESTAMP default now(),
    description varchar(1000),
    format varchar(30),
    structure varchar(200),
    duration varchar(200),
    price integer,
    language varchar(30),
    level varchar(30)
);

insert into course
(id,teacher_id,name,time)
values(1,1,'First course', '2024-02-10 04:40:44');

insert into course
(id,teacher_id,name,time)
values(2,1,'Second course', '2024-03-10 05:50:55');%   