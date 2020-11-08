---
title = "The Magical Switch in your Interface"
descr = "Some considerations on putting booleans in your interfaces."
url_friendly_name = "magical-switch-interface"
date = 2020-10-01
---

How many times have you come across a helper function or an endpoint, that does *almost* everything you need? There is just that *one* functionality that you need to add or change before the function or endpoint is perfect but existing code already depends on the function. Easy, you think, just add a boolean to the interface that controls which whether the old or the new functionality is used, and now both the old **and** the new functionality is supported. One year later, the new Software Engineer maintaining the project curses under their breath as they read the code. What happened?

What happened, is you created a magical switch in your code. Your new boolean acts a switch between two branches of functionalities. Later, as more code is added and changed to the different branches and more magical switches are added, the switch that initially was very straightforward in your head becomes more and more magical, until someone has to spend an hour just to figure out the implications of passing in true instead of false.

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

Now, your boss tells you to add functionality for savings accounts. You were taught to reuse code, so you decide to reuse the `withdrawCash` function like such:

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

The code works, you ship the functionality, and everybody is happening. Why is this code harder to maintain? Imagine you are the new maintainer and you had to do one of the following:

1. You are adding a third type of account, an investment account. How would you do so?
   * You could add another boolean, `isInvestmentAccount`. But what happens if `isSavingsAccount` and `isInvestmentAccount` are both true? Now this interface can possibly have undefined behavior, and another maintainer can possibly burn themselves.
   * You could change isSavingsAccount from a boolean to an enum, and the `if isSavings ...` block into a case switch. But now, you've simply added a branch to your magical switch.
1. You want to allow users to overdraft their checking account. Now, the `if amount < ...` check will only apply if the user does not have overdraft on.
    * Because the overdraft check is in `withdrawCash`, you are forced to modify the functionality of withdraw cash. Why not add another magical switch? You change the overdraft check to the following:

    ```
    if not isOverdraftAllowed && amount < account.getAmount() {
        throw Error
    }
    ```
    
    Great! But now, if isOverdraftAllowed is on AND isSavingsAccount is true, you now allow overdrafting on savings accounts. Oops.
1. You just want to call the function as-is.
    * Suppose you simply called the function with a simple comment above. It would look like this:

    ```
    // Withdraw cash from the savings account.
    withdrawCash(user, accountId, 500, true);
    ```

    The comment clearly says what's happening, and the name of the function `withdrawCash` gives some context to the magical number 500: you are withdrawing 500 units from this account. But if you don't haven't seen the implementation of `withdrawCash` before, you'd probably be scratching your head as to what the `true` value does. The interface forces the user to look into the implementation, and therefore fails its job of abstracting away complexity.
    
    * Suppose you were a bit savvier, and either named the parameter (whether using comments or if the language supports it) or used an enum. It would look like this: 

    ```
    withdrawCash(user, accountId, 500, SAVINGS).
    ```

    It's a lot more readable, to the point that you don't even need the comment anymore! But what happens if you need to add more magical switches? It may end up looking something like this:

    ```
    withdrawCash(user, accountId, 500, DOLLARS, SAVINGS, isOverdraftAllowed=false,      
                 accountOwnershipCheck=false)
    ```

    Now, your arguments are still understandable, but now there are too many parameters! Try to imagine the jumble of branches the function implementation would be like to take all these magical switches.

The issue with the magical switch is **abstraction**. A magical switch 


    


    