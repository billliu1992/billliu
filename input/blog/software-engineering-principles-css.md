---
title = "Applying Software Engineering Principles to CSS"
descr = "Making maintenance of CSS a bit more bearable by applying software engineering principles."
url_friendly_name = "software-engineering-principles-css"
date = 2020-11-28
---

CSS is the ugly step-child of front-end web development. The amount of jokes
and [memes][reddit] about how hard it can be are endless. Just vertically centering things require [instructions][instructions]. Given its difficulty, one may think working with CSS is just difficult by default. Can we apply some software engineering principles to make our lives maintaining CSS better?

## Tip: Make your HTML/CSS communicate your intentions

Making even a small change to the code can be hard if you have no idea what the
code is doing. Even worse, if you don't understand the code you're changing,
you're likely to duct-tape a solution onto the existing code, making the code
even harder to change in the future (aka tech debt). The solution is to make
your intentions clear in your code.

How well can CSS rules communicate intentions? Let's take a look at this example:

```
<style>
    .control {
        width: 300px;
    }
    .switch {
        margin-left: 200px;
        vertical-align: middle;
    }
</style>
<div class="control">
    <label for="mute-control" class="label">Mute</span>
    <input type="checkbox" id="mute-control" class="switch" />
</div>
```

What does the CSS say about how the page should look? Using `margin-left` to horizontally align the checkbox is very imperative: we are saying the element should be 200 pixels to the left, but we aren't saying why. Let's look at an alternative example:

```
<style>
    .control {
        width: 300px;
    }
    .switch {
        float: right;
    }
</style>
<div class="control">
    <label for="mute-control" class="label">Mute</span>
    <input type="checkbox" id="mute-control" class="switch" />
</div>
```

Here, we use `float: right` to communicate that the checkbox should be right-aligned.

This information is very helpful to the CSS interpreter. What if the label was changed to a longer text? If you were manually aligning it with pixels, you'd be forced to recompute how many pixels to shift to the right. You wouldn't need to do this if you simply told the CSS interpreter "make this element right-aligned." 

This information is also very helpful to a future maintainer. Having clear intentions help communicate when a CSS rule can be changed or removed. Otherwise, if you do not know why a CSS rule was used, you're likely to keep it around which can quickly balloon your CSS files.

The thing about communication is that it's highly dependent upon the parties communicating: what may be clear to one may not be clear to others. It may help to be open to feedback when something that is clear to you isn't clear to another maintainer. It may also be helpful to identify opportunities to teach others, so that what's clear to you becomes clear to others.

## Tip: Keep your HTML/CSS concise.

Have you ever heard a long and rambling story? Long stories aren't by default bad, but a lot of non-constructive additions can make the point hard to keep track of. Like a bad story, HTML/CSS that is unnecessarily large can be hard to wrangle.

One way CSS can become unnecessarily large is if there are a lot of unused rules. Digging through a lot of rules is hard, and unused rules just adds to the overhead. We briefly mentioned a solution in the previous tip, but you can also consider static analysis.

Another way CSS can become unnecessarily large is if we have a lot of wrapper elements. Let's take an extreme example:

```
<style>
  .container1 {
      background-color: red;
      color: blue;
      display: flex;
      height: 50px;
  }
  .container2 {
      font-family: serif;
      height: 30x;
  }
  .container3 {
      font-family: sans-serif;
      padding: 10px;
      position: absolute;
  }
  .container4 {
      position: absolute;
      top: 20px;
  }
  .text {
      font-size: 12px;
  }
</style>

<div class="container1">
    <div class="container2">
        <span class="text">Hello</span>
    </div>
    <div class="container3">
        <div class="container4">
            <span class="text">Bye</span>
        </div>
    </div>
</div>
```

Figuring out when a child is inheriting a parent rule can be hard, but it can be easier if you don't have a lot of wrapper elements. The more wrapper elements you have, the more places you need to watch out for an inherited rule.

## Tip: Keep future changes in mind

CSS by design allows ancestor elements to affect the style of child elements.
Sometimes, a rule on an ancestor can make it a lot harder to change a child
element. This can be frustrating to deal with, when a seemingly simple task
suddenly takes you a lot longer becauase of a style set in the parent.

Example of things that can be frustrating to workaround:

 * A rule with the `!important` tag.
 * Setting `position: absolute` on an element causes all absolutely positioned children to be positioned relative to said element.
 * Setting the `z-index`. At best, this creates an "arms race" of increasing `z-index` values, and at worst creates UI bugs.
 * Setting `overflow: hidden` means adding a larger child will not cause the parent element to grow as expected, but instead be cut-off.

None of the above examples are necessarily bad practices. However, under certain circumstances, they can cause future changes to be harder. Because of this, it may make sense to try to avoid them if possible.

One of my recent reads is [The Pragmatic Programmer][prag], which says well designed software is *easier to change*. I really liked the reasons they gave in the book, and would highly recommend it to other software engineers.

Software engineering principles are general, and there can be a lot of discussion on how best to implement them. How do you apply software engineering principles to CSS? How do you keep your CSS maintainable? Let me know what you think.

[instructions]: http://howtocenterincss.com/
[reddit]: https://www.reddit.com/r/ProgrammerHumor/comments/a6rkoa/the_pains_of_css/?ref=share&ref_source=link
[prag]: https://pragprog.com/titles/tpp20/the-pragmatic-programmer-20th-anniversary-edition/
