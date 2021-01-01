---
title = "Thoughtfully Evolving Interfaces"
descr = "Some things to think about when adding functionality to an existing codebase."
url_friendly_name = "thoughtfully-evolving-interfaces"
date = 2020-10-01
---

How many times have you come across a helper function or an endpoint, that does *almost* everything you need? Extending existing code to fit a new use-case can be awkward, because the existing code rarely fits as-is so we need to add to it in a way that it serves both the old and new use-case. If we aren't too careful with how we extend our code, we can quickly create a few unreadable Frankenstein's monsters.

Let's take a concrete example. You work for a bank that currently offers checking accounts. The withdrawal code looks like this: 

```
function withdrawCash(user, accountId, amount) {
    if not user.hasAccount(accountId) {
        throw Error
    }

    var account = user.getCheckingAccount(accoundId);

    if amount < account.getAmount() {
        throw Error
    }

    account.withdrawAmount(amount)
}
```

Now, you need to add functionality for savings accounts. How would you do so? You may be tempted to reuse the `withdrawCash` function like such:

```
function withdrawCash(user, accountId, amount, isSavingsAccount) {
    if not user.hasAccount(accountId) {
        throw Error
    }

    var account
    if isSavings {
        account = user.getSavingsAccount(accountId)
    } else {
        account = user.getCheckingAccount(accountId)
    }

    if amount < account.getAmount() {
        throw Error
    }

    account.withdrawAmount(amount)
}
```

This code works, but now suppose you add support for an investment account, how would you do so? Following the same pattern, we would add a new parameter like this: `withdrawCash(user, accountId, amount, isSavingsAccount, isInvestmentAccount)`. However, this is not ideal. Firstly, the number of parameters is ballooning, to the point that using the function is becoming unwieldy. Secondly, setting both `isSavingsAccount` and `isInvestmentAccount` to `true` would likely cause an error so now clients can potentially cut themselves on the interface. How can we make this code better?

## Easy wins

One easy way we can make this better is to simply make the account type an enum. That way, the interface becomes `withdrawCash(user, accountId, amount, accountType)`. We no longer have a long argument list, and we no longer let the user select multiple account types.

One best practice with Protobuf is to never use a boolean to handle a value that we may one day want to extend [(see Google recommendations)][goog-proto]. While we aren't dealing with protobufs in this example, you can see why a boolean can be awkward in some cases: your interface can quickly balloon and introduce sharp edges.

## Move the account type abstraction down the call stack

Can we eliminate the account type parameter altogether? We can do so if we rethink our layers of abstraction. Our user can simply fetch a generic account, so we now have the following code:

```
function withdrawCash(user, accountId, amount) {
    if not user.hasAccount(accountId) {
        throw Error
    }

    var account = user.getAccount(accountId)

    if amount < account.getAmount() {
        throw Error
    }

    account.withdrawAmount(amount)
}
```

We have now moved the concept of an account type down the call stack and into the call to the `user` object. The function is now a lot easier to read and use. For many use-cases, this is a perfect stopping point.

However, let's look at some consequences of getting to this point.

The most obvious is that we need to make some code modifications in `user`, and potentially the different account types to get to this point, which can be cost-prohibitive in a large code base. That is why it's so important to get the interface right the first time.

Another consequence is that the `user` now needs to do the account type lookup. If the lookup is not cheap, we now force every call to withdrawCash to incur this lookup cost. Furthermore, the client can no longer control the account type. This can be okay and even ideal in some cases, but if we actually need this flexibility in the future we can actually be creating more work for ourselves.

It's clear that moving complexity down the call stack can be helpful, but how do we handle the case where we need more flexibility?

## Move the account type abstraction up the call stack

We can force the client to handle the complexity of account type. This gives the caller of `withdrawCash` the flexibility to handle different types of account types separately. Our new code for `withdrawCash` will be as such:

```
function withdrawCash(account, amount) {
    if amount < account.getAmount() {
        throw Error
    }

    account.withdrawAmount(amount)
}
```

It's even simpler than moving the abstraction down the stack! This can also be great if you wanted to implement specific funcionality for certain account types, like overdraft for checking accounts only.

However, now every single caller now needs to do the work that you've moved out of `withdrawCash`. That can mean a lot of code duplication. We can also take this example to it's logical extreme: we remove everything out of the function and inline everything. That probably wouldn't make for very maintainable code if `withdrawCash` is called in a lot of places.

Have you ever used a complex API? It isn't a lot of fun, and from my personal experience, I always think "why can't you just do this for me?" It's clear that there are some cases that we do not want too much complexity.

## Putting it all together

Ultimately, how a codebase is evolved will determine how nice it is to work on it. Having a codebase be easy to work on without too much technical debt requires thoughtful extension.

Unfortunately, there is no one size fits all: if we lean on abstraction, we can block ourselves from making the correct modifications later, but if we lean on complexity, we stop providing as much value to our clients. The optimal balance is highly situation dependent.

I hope you found this somewhat useful.

[goog-proto]: https://cloud.google.com/apis/design/design_patterns#bool_vs_enum_vs_string