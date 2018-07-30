# Contributing to Imgly
Imgly is open to accept any contributions if it improves from it, in any shape or form. This document gives you some
simple guidelines to help working on the project.

## Submitting bug reports and feature requests
Imgly is maintained on Github and only use the issue tracker over there, please use the Github repository to create
your issue and/or feature request.

When reporting a bug make sure to include your Rust version and all the steps to reproduce this bug.

When making a feature request make it clear what the problem you intend to solve with it, include both all the improvements
and disadvantages with it. Make sure to be as complete as possible and to think about compatibility in the future.

## Setting up your environment
To get started clone the repository with `git clone git@github.com:laetificat/imgly.git`, then add a file named `.env`
to the root directory of the project.
Inside the .env file you want to add these lines (edit where necessary):
```
DATABASE_URL=postgres://user:password@host/imgly
BASE_URL=http://your_url:your_port
```