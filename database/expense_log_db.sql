------------------------------------------
-- CREATE TABLES
------------------------------------------

create table app_user (
   id          serial       not null,
   username    varchar(100) not null,
   email       varchar(100) not null,
   password    varchar(20)  not null,
   admin_user  boolean      not null,
   constraint  pk_app_user  primary key (id)
);

create table expense_type (
   id          serial          not null,
   title       varchar(100)    not null,
   description varchar(255)    not null,
   constraint  pk_expense_type primary key (id)
);

create table expense (
   id              serial        not null,
   expense_date    date          not null,
   expense_type_id int           not null,
   amount          decimal(12,2) not null,
   description     varchar(255)  not null,
   app_user_id     int           not null,
   constraint      pk_expense    primary key (id)
);

create index index_expense_type_id on expense (expense_type_id ASC);
create index index_app_user_id on expense (app_user_id ASC);

create table expense_alert (
   id           serial           not null,
   day_of_month int              not null,
   total_value  decimal(12,2)    not null,
   constraint   pk_expense_alert primary key (id)
);

------------------------------------------
-- POPULATE TABLES
------------------------------------------

INSERT INTO app_user (id, username, email, "password", admin_user) VALUES(nextval('app_user_id_seq'::regclass), 'Mickey Mouse', 'mickey.mouse@cartoon.net', '123', true);
INSERT INTO app_user (id, username, email, "password", admin_user) VALUES(nextval('app_user_id_seq'::regclass), 'Donald Duck', 'donald.duck@cartoon.net', '123', false);

INSERT INTO expense_type (id, title, description) VALUES(nextval('expense_type_id_seq'::regclass), 'Electricity', '');
INSERT INTO expense_type (id, title, description) VALUES(nextval('expense_type_id_seq'::regclass), 'Heating', '');
INSERT INTO expense_type (id, title, description) VALUES(nextval('expense_type_id_seq'::regclass), 'Water', '');
INSERT INTO expense_type (id, title, description) VALUES(nextval('expense_type_id_seq'::regclass), 'Mobile Phone', '');
INSERT INTO expense_type (id, title, description) VALUES(nextval('expense_type_id_seq'::regclass), 'Internet', '');
INSERT INTO expense_type (id, title, description) VALUES(nextval('expense_type_id_seq'::regclass), 'Gasoline', '');
INSERT INTO expense_type (id, title, description) VALUES(nextval('expense_type_id_seq'::regclass), 'Rent', '');
INSERT INTO expense_type (id, title, description) VALUES(nextval('expense_type_id_seq'::regclass), 'Grocery', '');
INSERT INTO expense_type (id, title, description) VALUES(nextval('expense_type_id_seq'::regclass), 'Fitness Club', '');
INSERT INTO expense_type (id, title, description) VALUES(nextval('expense_type_id_seq'::regclass), 'Cinema', '');
INSERT INTO expense_type (id, title, description) VALUES(nextval('expense_type_id_seq'::regclass), 'Meal', '');
INSERT INTO expense_type (id, title, description) VALUES(nextval('expense_type_id_seq'::regclass), 'Coffee', '');

INSERT INTO expense (id, expense_date, expense_type_id, amount, description, app_user_id) VALUES(nextval('expense_id_seq'::regclass), '2024-02-05', 7, 550, '', 1);
INSERT INTO expense (id, expense_date, expense_type_id, amount, description, app_user_id) VALUES(nextval('expense_id_seq'::regclass), '2024-02-05', 11, 12, '', 1);
INSERT INTO expense (id, expense_date, expense_type_id, amount, description, app_user_id) VALUES(nextval('expense_id_seq'::regclass), '2024-02-06', 6, 53, '', 1);
INSERT INTO expense (id, expense_date, expense_type_id, amount, description, app_user_id) VALUES(nextval('expense_id_seq'::regclass), '2024-02-06', 8, 73.5, '', 1);
INSERT INTO expense (id, expense_date, expense_type_id, amount, description, app_user_id) VALUES(nextval('expense_id_seq'::regclass), '2024-02-06', 11, 9.5, '', 2);
INSERT INTO expense (id, expense_date, expense_type_id, amount, description, app_user_id) VALUES(nextval('expense_id_seq'::regclass), '2024-02-08', 9, 43, '', 1);
INSERT INTO expense (id, expense_date, expense_type_id, amount, description, app_user_id) VALUES(nextval('expense_id_seq'::regclass), '2024-02-09', 11, 10.3, '', 1);
INSERT INTO expense (id, expense_date, expense_type_id, amount, description, app_user_id) VALUES(nextval('expense_id_seq'::regclass), '2024-02-09', 11, 11.4, '', 2);
INSERT INTO expense (id, expense_date, expense_type_id, amount, description, app_user_id) VALUES(nextval('expense_id_seq'::regclass), '2024-02-15', 1, 55.36, '', 1);
INSERT INTO expense (id, expense_date, expense_type_id, amount, description, app_user_id) VALUES(nextval('expense_id_seq'::regclass), '2024-02-16', 12, 1.5, '', 1);
INSERT INTO expense (id, expense_date, expense_type_id, amount, description, app_user_id) VALUES(nextval('expense_id_seq'::regclass), '2024-02-20', 2, 49.7, '', 1);
INSERT INTO expense (id, expense_date, expense_type_id, amount, description, app_user_id) VALUES(nextval('expense_id_seq'::regclass), '2024-02-21', 3, 39.9, '', 1);
INSERT INTO expense (id, expense_date, expense_type_id, amount, description, app_user_id) VALUES(nextval('expense_id_seq'::regclass), '2024-02-21', 11, 12.2, '', 1);
INSERT INTO expense (id, expense_date, expense_type_id, amount, description, app_user_id) VALUES(nextval('expense_id_seq'::regclass), '2024-02-21', 11, 8.9, '', 2);
INSERT INTO expense (id, expense_date, expense_type_id, amount, description, app_user_id) VALUES(nextval('expense_id_seq'::regclass), '2024-02-22', 8, 135.87, '', 1);
INSERT INTO expense (id, expense_date, expense_type_id, amount, description, app_user_id) VALUES(nextval('expense_id_seq'::regclass), '2024-02-23', 10, 15, '', 1);
