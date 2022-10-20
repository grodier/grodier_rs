# Here Goes Nothing

Readme's are great entry point for documentation into a repo: what it is, how a user can interact with it, etc.

But here's the fun thing: there are NO rulez!

So here's what I'm going to do with this README (at least for the time being). I want a running journal of things I'm doing with this repo and why I'm doing it. Yeah some of that may be captured in the commits/pull requests, but I want to add a little more context. And then hopefully I can look back and evaluate a pragmatic process or critique a messy mind.

## Starting out

Ok so what is this repo? Well it's a personal site. I want to develop the backend in Rust for a couple reasons. First, I want an excuse to play with Rust (and frankly that should be good enough). But I also want to explore web fundamentals without the burden of JavaScript getting in the way.

Most of my server experience has been in an isomorphic rendering type of paradigm (ssr with client hydration). Usually that work is closely coupled to a frontend framework (cough - React - cough). However, a framework brings a lot of JavaScript and a lot of room to make mistakes. So my goal with a server is to generate html on the server side and just deliver that. And then sprinkle in JavaScript where's needed. And then I want to feel the pain that caused frameworks to be built with in the first place. BRING IT ON!

But in doing so I want to see where that pain point actually is. Too often we reach for a framework as the defacto approach, but are we reaching for it to soon? What's wrong with just html? What would JavaScript sprinkles look like if it were agnostic of the backend system? I'm not quite positive yet (other than guesses), but I want to find out.

Now if this is just starting out as a personal site, then why even have a server. Why not just statically host html - maybe generated with something like 11ty? Well, back to first reason of wanting to play with Rust. But also I like the idea of exploring dynamic server rendered content. What will I build that actually takes advantage of it? Not sure yet, but I am determined.

I could use a serverless approach (Because the web is just an api to functions that returns content and "serverless" fits well with that model). And I like the idea of micro services as well. BUT I again want to start with something larger and find where those boundaries for separating things are and how to refactor to that when the PAIN is felt. Maybe I'm crazy, but this feels like an exercise in masochism. Or maybe I'm just curious (my name is George after all).

Anyways, I can't quite quit JavaScript yet. The package.json is one of my favorite parts of the JavaScript ecosystem. Seeing commands right there in a script block and knowing how to invoke them makes repos easy to navigate, learn, and provides a sense of comfort. And just because I'm quiting JavaScript on my backend, doesn't mean I won't have any on the frontend. And building for the web means you will probably be tied to the JavaScript ecosystem in someway (transpiling TypeScript, web components, generating css, e2e testing, etc). So I want to be set up to build for that as well. In fact I want to build it all in one step. So I'm opting to use turbo repo and setting up the project as a monorepo with my frontend js/css packages and rust server.

## Setting things up with actix-web (10/19/22)

Using actix web and going through their hello world. Using that because I've come across it in a couple tutorials (zero2prod) and am slightly used to it and think (for the time being) that I prefer the api to rocket. Actix-web feels (and it's just a gut feeling that I haven't diven into too much) more low level and ripe for exploring. Which is just what I want. I honestly could be wrong here. But a) actix-web appears to be a suitable option b) it's super fast c) I have a hint of familiarity with it so let's be pragmatic.

Anyways, going through their hello world to make sure I understand their intro api's like I think I might.
