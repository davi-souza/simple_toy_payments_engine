# Simple Toy Payments Engine

By Davi Souza

## Setup

First of all, you need to add a `transactions.csv` file at the root of this project.

This `transactions.csv` file should be be like

```csv
type,client,tx,amount
deposit,1,1,5.0
deposit,2,2,6.0
dispute,1,1,
chargeback,1,1,
dispute,2,2,
resolve,2,2,
```

### Docker

Pull a docker image to run the project. A suggestion is to run

```bash
docker pull rust:1.55.0-slim-bullseye
```

## Run the project

To run the project, you should run the following command

```bash
cargo run -q -- transactions.csv > accounts.csv
```

It will process the `transactions.csv` file and generate an `accounts.csv` file with the results of the processing.

### Docker

To run in the docker containter, you can run the following command

```bash
docker run -it --rm -v $(pwd):/project -w /project --name project rust:1.55.0-slim-bullseye cargo run -q -- transactions.csv > accounts.csv
```

And it will also generate an `accounts.csv` file with the results of the processing.

OBS: this might take a while because it is going to download all the crates.

## Output

An `accounts.csv` file is going to generated. It should look like this

```csv
id,available,held,total,locked
1,1.5,0,1.5000,false
2,2,0,2.0000,false
```

## Notes

### Assumptions

I made some assumptions in order to achieve proper results.

One of the assumptions is that the transaction type matters when I'm processing a `dispute` row of the `transactions.csv` file.

When I'm disputing a `deposit` transaction, I decrease the `available` funds and I increase the `held` funds of the account by the `amount` absolute value of the original transaction. Because the end result of reverting the original transaction (of type `deposit`) is that the account is going to have less funds available (and therefore total funds) than it had before the dispute.

And when I'm disputing a `withdrawal` transaction, I increase the `available` funds and I decrease the `held` funds of the account by the `amount` absolute value of the original transaction. Because the end result of reverting the original transaction (of type `withdrawal`) is that the account is going to have more funds available (and therefore total funds) than it had before the dispute.