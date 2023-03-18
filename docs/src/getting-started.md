## Getting Started
Currently, the only way to get started with the project is to clone, fork or download the repository.


## Requirements 

The following are required to run the application in development or in production.

- [Node.js](https://nodejs.org/) >= 16.0.0
- [MySQL](https://www.mysql.com/) >= 8.0.0 or a preferred SQL server such as [PostgreSQL](https://www.postgresql.org/) or [SQLite](https://www.sqlite.org/index.html)  
[TypeScript](https://www.typescriptlang.org/) >= 4.3.5 
- [Yarn](https://yarnpkg.com/) >= 1.22.0 or a preferred Node package manager such as [npm](https://www.npmjs.com/) or [pnpm](https://pnpm.js.org/)
- [Docker](https://www.docker.com/) >= 20.10.0 (optional)




## Installation (development)
[Molybdenum](https://opeolluwa.github.io/molybdenum/) is built with using [yarn](https://yarnpkg.com/) for package management. To use other Node package manager such as [npm](https://www.npmjs.com/) or [pnpm](https://pnpm.js.org/). 
Delete the `yarn.lock` file and run `npm install` or `pnpm install` to install the dependencies. Also, use your preferred package manager in place of `yarn` to execute
 the following commands.

1. Clone the repository and run `yarn` to install the dependencies.
2. Create a `.env` file in the root directory and populate it using the `.env.example` file.
3. Run `yarn start` to start the server.
4. Run `yarn test` to run the tests.
5. Run `yarn run build` to build the application for production.

## Demo

Try out a live deployment [demo](example.com) of the application.

## Documentation
Check out the [documentation](https://opeolluwa.github.io/molybdenum/) for more information.
