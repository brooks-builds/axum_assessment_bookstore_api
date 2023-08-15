# Bookstore API

We've been hired to create an API for a bookstore! We'll need to set up a server along with a database and seed it with some books and authors so our client can start selling their books.

## Objectives

This assessment is checking the following skills

- Connecting to a database
- Setting up a database with Migrations
- Seeding initial data
- CRUD
  - Create
    - Inserting a row into a table
    - Inserting a join into a table
  - Reading
    - Getting one item
    - Getting many items
    - Getting items with their referenced items
  - Updating
  - Deleting
    - With cascade

Since this is an assessment we aren't going to be implementing auth for this API, we're focused on the database connection and CRUDing the data.

## Instructions

Clone and checkout the assessment to your local system or use a VM like GitHub Codespaces.

Read the objectives and rubric to see what you need to do to pass the assessment.

Spin up a database. A database is provided with Docker using the command `docker compose up`.

Open the code in an editor and review what it looks like.

Since the assessment is only testing for the skills listed above, some of the application has already been created. All you need to do is update the code so that the tests pass. Comments have been added to the code to give guidance as to which files need to be updated. Some files may need to be created.

The check.sh script will run the tests, and check the code for linting and code formatting warnings. To pass, the check.sh script must pass. You can run this yourself to see if everything is passing. The output of the commands are appended to a file named check.out.

To run the tests on their own run the command `cargo test`. This can help see what remains to be done without having to run check.sh and look through the check.out file.

A solution for this is on the Solution branch. You can check this out locally or view it on GitHub to see one way to solve the assessment and pass it

If you want to inspect the database itself you have a couple of options.

- Connect directly using PSQL or a GUI database tool from your system. By default the database username is **postgres**, and password is **keyboardcat**. You can change this in the [compose.yaml](compose.yaml) file.
- Connect to the Docker container directly and run PSQL inside it with the command `docker compose exec database psql -U postgres` from the root of this project.

## Rubric

To pass this assessment the following needs to be done

- Connection to database set up
- Database migrations creating tables for authors, books, and book_authors
- Database seeds set up to create the following authors and books
- Route created to insert an author
- Route created to get one author
- Route created to get all authors
- Route created to update an author
- Route created to delete an author
- Route created to insert a book
- Route created to get one book
- Route created to get all books
- Route created to update a book
- Route created to delete a book
- Route created to associate book with author
- check.sh script is passing

## Tables to Create

### authors

| PK | column name | column type |
| -- | ----------- | ----------- |
| *  | id          | int         |
|    | name        | string      |

### books

| PK | column name | column type |
----------------------------------
| *  | id          | int         |
|    | name        | string      |
|    | price       | int         |
|    | in_stock    | bool        |


### book_authors

| PK | column name | column type |
----------------------------------
| *  | author_id   | int         |
| *  | book_id     | int         |

## Seeds

### Authors

| id | name           |
-----------------------
| 1  | Unpublished    |
| 2  | One Book       |
| 3  | Multiple Books |

### Books

| id | name             | price | in_stock |
--------------------------------------------
| 1  | Free Book        | 0     | true     |
| 2  | Expensive Book   | 10000 | true     |
| 3  | Unavailable Book | 1400  | false    |

### Books Authors

| author_id | book_id |
-----------------------
| 2         | 1       |
| 3         | 2       |
| 3         | 3       |
