# Getting Started


## Create a Cargo project

First we need to ensure that you have `rustup`, Rust's toolchain for developing rust programs. Install if you dont have using this [link here]().

We can now create a new `Cargo` project by writing this on the terminal 

```shell
cargo new --bin project-name
cd project-name
```

So that is the project we are going to use to consume the [IntaSend](https://www.intasend.com) payment gateway using the `intasend` cargo Rust crate.  

## Sandbox Credentials (API Keys)

We are now required to have our API keys in order consume the payment gateway. 
If you dont have an account you can [signup](https://sandbox.intasend.com) or else login into your [sandbox intasend account](https://sandbox.intasend.com) and grab the **publishable key** and **secret key** on the credentials page. 

Create a new file called `.env` in our project folder and add them in the following format

```shell
# IntaSend Env Config
INTASEND_PUBLIC_KEY=YOUR_PUBLIC_KEY_HERE
INTASEND_SECRET_KEY=YOUR_SECRET_KEY_HERE
INTASEND_TEST_MODE=true

```

## Production Credentials (API Keys)

For prodction scenarios, we are required to have our API keys in order consume the payment gateway. 
If you dont have an account you can [signup](https://payment.intasend.com) or else login into your [payment intasend account](https://payment.intasend.com) and grab the **publishable key** and **secret key** on the credentials page. 

Create a new file called `.env` in our project folder and add them in the following format

```shell
# IntaSend Env Config
INTASEND_PUBLIC_KEY=YOUR_PUBLIC_KEY_HERE
INTASEND_SECRET_KEY=YOUR_SECRET_KEY_HERE
INTASEND_TEST_MODE=false

```
