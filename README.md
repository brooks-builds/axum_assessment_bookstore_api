# Bookstore API

We've been hired to create an API for a bookstore! We'll need to set up a server along with a database and seed it with some books and authors so our client can start selling their books.

## Objectives

This assessment is checking the following skills

- Using SeaORM to connect to a database
- Setting up a database with Migrations using SeaORM
- Seeding initial data with SeaORM
- CRUD with SeaORM
  - Simple queries
  - Many to Many join
  - Inserting
  - Updating
  - Deleting

Since this is an assessment we aren't going to be implementing auth for this API, we're focused on 

## Instructions

Clone and checkout the assessment to your local system or use a VM like GitHub Codespaces.

Read the objectives and rubric to see what you need to do to pass the assessment.

Open the code in an editor and review what it looks like.

Since the assessment is only testing for the skills listed above, some of the application has already been created. All you need to do is update the code so that the tests pass. Comments have been added to the code to give guidance as to which files need to be updated. Some files may need to be created.

The check.sh script will run the tests, and check the code for linting and code formatting warnings. To pass the check.sh script must pass. You can run this yourself to see if everything is passing. The output of the commands are appended to a file check.out.

To run the tests on their own run the command `cargo test`. This can help see what remains to be done without having to run check.sh and look through the check.out file.

A solution for this is on the Solution branch. You can check this out locally or view it on GitHub to see one way to solve the assessment and pass it

In development, spin up a database using docker with the command `docker compose up`.

If you want to inspect the database itself you have a couple of options.

- Connect directly using PSQL or a GUI database tool from your system. By default the database username is postgres, and password is keyboardcat. You can change this in the [compose.yaml](compose.yaml) file.
- Connect to the Docker container directly and run PSQL inside it with the command `docker compose exec database psql -U postgres`

## Rubric

To pass this assessment the following needs to be done

- task
- task
- task
- check.sh script is passing

