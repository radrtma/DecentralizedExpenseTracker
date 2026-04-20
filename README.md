# Decentralized Expense Tracker

## Overview

Decentralized Expense Tracker is a simple smart contract built on the Stellar network using Soroban. It demonstrates how financial data such as income and expenses can be stored and managed directly on-chain without relying on a traditional backend server.

This project focuses on the fundamentals of smart contract development, including data storage, retrieval, and basic financial logic in a decentralized environment.

---

## Features

* Store income value
* Store expense value
* Retrieve income and expense
* Calculate balance (income - expense)

---

## How It Works

This smart contract uses on-chain storage to manage two values:

* `income`
* `expense`

The balance is calculated dynamically using:

```text id="j3w6l9"
balance = income - expense
```

---

## Functions

### set_income(amount: i128)

Stores the income value in contract storage.

### get_income() -> i128

Returns the stored income value.

### set_expense(amount: i128)

Stores the expense value in contract storage.

### get_expense() -> i128

Returns the stored expense value.

### get_balance() -> i128

Calculates and returns the current balance by subtracting expense from income.

---

## Deployment

The contract is deployed on Stellar Testnet.

Contract ID:

```text id="u6xq4y"
CB2DVPEZ22YHR2RPBPRNGQNWF64ATJCJ2X2ON5QBJCJDOT6EWJFGMU3T
```

---

## Example Usage

```text id="1m4o9h"
set_income(5000)
set_expense(2000)
get_balance()  // Result: 3000
```

---

## Why I Built This

I built this project as part of a workshop to understand how smart contracts work on the Stellar network using Soroban. Instead of focusing only on theory, I wanted to directly implement a simple but meaningful use case: tracking income and expenses.

This project also represents my first step into backend development on blockchain. As someone with a stronger background in frontend development, I used this opportunity to improve my understanding of data flow, storage, and logic in a decentralized system.

Although the current implementation is still basic, it serves as a foundation for a more complete financial application. In the future, I plan to extend this project by adding support for multiple transactions, categorization, and split bill features.

---

## Future Improvements

* Support multiple transactions (list-based storage)
* Add transaction categories
* Implement split bill functionality
* Add user-based data using wallet addresses

---

## Notes

This project is a basic implementation intended for learning purposes and to demonstrate fundamental concepts of Soroban smart contracts.
