---
title = "Applying Software Engineering Principles to CSS"
descr = "Making maintenance of CSS a bit more bearable by applying software engineering principles."
url_friendly_name = "software-engineering-principles-css"
date = 2020-11-28
---

CSS is the ugly step-child of front-end web development. The amount of jokes
and [memes][reddit] about how hard it can be are endless. Just vertically centering things require [instructions][instructions]. Given its difficulty, one may think working with CSS is just difficult by default. Can we apply some software engineering principles to make our lives maintaining CSS better?

Over the 5 years I've spent working on web pages, an embarassing amount of hours
were spent fiddling with CSS rules for a "simple" change. A lot of good developers I know simply take it for granted that CSS will be difficult, and therefore there is not a lot of gain in writing principled CSS. However, over the years I have found that sometimes, CSS isn't actually painful to work with. Here, I reflect on the times that CSS isn't painful to work with, and attempt to distill it as generic tips.

## Tip: Make your HTML/CSS communicate your intentions

Making even a small change to the code can be hard if you have no idea what the
code is doing. Even worse, if you don't understand the code you're changing,
you're likely to duct-tape a solution onto the existing code, making the code
even harder to change in the future (aka tech debt). The solution is to make
your intentions clear in your code.

You can make your CSS rules accurately reflect your intentions. For example,
if you are trying to right align something in CSS, don't hand-calculate the
parent width and set an appropriate `margin-left`. Nothing about `margin-left` says that changing its value will mess up the alignment of the page. Instead, you can use `text-align: right`, `float: right`, or flexbox.

Another reason this is important for maintenance is that clear intentions also help communicate when a CSS rule can be *removed*. Otherwise, if you do not know what a rule is doing, you're likely to keep it around which can quickly balloon your CSS files.

This also applies to HTML. Because CSS styles HTML, we often find ourselves
adding wrapper elements in HTML so they can be correctly styled by CSS.
When we have a lot of wrapper elements in the HTML, it can be hard to figure
out which wrapper element corresponds to which part of the UI.

Instead, the HTML should accurately reflect your intentions with the structure
of the information on the page. Specifically, a future maintainer should be able
to go into your HTML and know which elements correspond with what part of the UI. This is much harder when there are a lot of extraneous elements used solely for styling, but if we cut down on these extraneous elements, any maintenance
on the HTML and CSS becomes a lot easier.

## Tip: Keep future changes in mind

CSS by design allows ancestor elements to affect the style of child elements.
Sometimes, a rule on an ancestor can make it a lot harder to change a child
element. This can be frustrating to deal with, when a seemingly simple task
suddenly takes you a lot longer.

For example, if you are styling a self-contained component that needs to
be absolutely positioned with `position: absolute`, but it has a parent element
that has `position: absolute`, you will only be able to absolutely position
the element within the parent element, and not the page as a whole. To fix this,
you either need to change the parent element to not set `position: absolute` or
move your component outside of the parent element; both nontrivial tasks.

Unfortunately, by allowing style on ancestor elements to affect child elements,
CSS makes it very hard to see exactly where all the pitfalls are until you
actually begin implementation. In that case, it's important to think about
potential pitfalls for future maintainers. For example, do not set
`position: absolute` except on relatively isolated components.

Another case where this can be an issue is when reusing classes. A lot of times,
when we reuse a class that's already being used elsewhere, we find that the
class does not completely fit our need. In that case, we can either add another
class to our new element, or we need to refactor the style on the old elements.
If we keep adding on a new class, our CSS quickly gets cluttered, but we can easily lose a lot of time if we decided to refactor the parent styles.

In this case we find that the reuse of CSS follows some of the same principles
of reusing code: if the lines of abstraction are not well thought out, then
reusing the code can be more of a hindrance than a help. Specifically, whereas
in code abstraction is achieved via an interface, in CSS we think about what is
inherited and what is set. For example, if we want our common class to fit
inside parent elements with varying widths, we probably shouldn't set a specific
width. On the flip side, if we wanted to make sure our text is blue, we should
set a `color` value.


[instructions]: http://howtocenterincss.com/
[reddit]: https://www.reddit.com/r/ProgrammerHumor/comments/a6rkoa/the_pains_of_css/?ref=share&ref_source=link
[prag]: https://pragprog.com/titles/tpp20/the-pragmatic-programmer-20th-anniversary-edition/
