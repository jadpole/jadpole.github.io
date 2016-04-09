---
layout: post
title:  "Introducing Arcanium"
categories: elm
---

Lately, I've been working with a friend on an online, forum-based RPG called
_Arcanium_. This is a single-page application written in Elm and PHP, and as I
progressed, I realised that there wasn't much documentation on the subject.
That is what those blog posts aim to fix.

This is not a tutorial, but rather a series of articles describing some
techniques that we've used when writing the application and putting those in the
real-world context of Arcanium. However, I believe that this will make it better
for __TODO__.

In this introduction, we won't write much code, but instead prepare our project
and install the necessary dependencies.

Let's get started!


## Setup and dependencies

For this SPA, we decided to use the [Elm](http://elm-lang.org/) programming
language and tools, [XAMPP for Linux](https://www.apachefriends.org/download.html)
with the [Composer](https://getcomposer.org/) dependency manager and a few
[Polymer](https://www.polymer-project.org/1.0/) components managed with
[Bower](http://bower.io/). We also had to use native modules to implement some
missing functionalities.

We started with the typical structure that I use for this kind of project:

```
```

Let's see what we got there in some detail:

| File/Directory          | Role |
|:-----------------------:|------|
| .gitignore              | Typically ignored files
| .htaccess               | Redirect all to `index.php`
| build.sh                | Download and build the dependencies
| index.php               | The client's `main` file
| app/                    | The client-facing API and resources
| app/bower.json          | Polymer components
| backend/                | Server-side logic which is inaccessible to the client
| backend/composer.json   | PHP dependencies
| frontend/               | Elm code for the client
| frontend/Visitor.elm    | The main Elm file before the user logged in

```sh
##? .gitignore

app/bower_components
app/build

backend/composer.lock
backend/vendor

frontend/*.js
frontend/elm-stuff
```

```sh
##? .htaccess

Options +FollowSymLinks

# Redirect requests without extension to index.php
RewriteEngine On
RewriteCond %{REQUEST_FILENAME} !-d
RewriteCond %{REQUEST_FILENAME} !-f
RewriteCond %{REQUEST_URI} !\.(gif|jpg|png|php|css|js|ttf|otf)$
RewriteRule (.+) index.php [L,QSA]
```

```json
//? app/bower.json

{
  "name": "arcanium-client",
  "authors": [
    "Jessy Pelletier-Lemire <jessyplemire@gmail.com>"
  ],
  "description": "A client for the Arcanium forum-based RPG",
  "main": "",
  "moduleType": [],
  "license": "MIT",
  "homepage": "",
  "ignore": [
    "**/.*",
    "node_modules",
    "bower_components",
    "test",
    "tests"
  ],
  "dependencies": {
    "iron-autogrow-textarea": "PolymerElements/iron-autogrow-textarea#^1.0.0",
    "iron-a11y-announcer": "PolymerElements/iron-a11y-announcer#^1.0.0",
    "iron-fit-behavior": "PolymerElements/iron-fit-behavior#^1.0.0",
    "iron-overlay-behavior": "PolymerElements/iron-overlay-behavior#^1.0.9",
    "paper-button": "PolymerElements/paper-button#~1.0.11",
    "paper-fab": "PolymerElements/paper-fab#~1.1.2",
    "paper-toast": "PolymerElements/paper-toast#~1.1.1"
  }
}
```

```json
//? backend/composer.json

{
    "require": {
        "catfan/Medoo": "^1.0"
    }
}
```

We'll take care of writing other files in the next article. For the moment,
let's install all of this.


## Building the app

Here's the script that we'll use for building our app from now on:

```sh
# Build for the first time, installing the dependencies
if [ "$1" = "init" ]; then
    # Install Polymer components
    cd app
    bower install
    cd ..

    # Install PHP dependencies
    cd backend
    composer.phar install
    cd ..

    # Install Elm dependencies
    cd frontend
    elm-package install
    cd ..
fi

# Add a directory for the compiled Elm code
if [ ! -d "app/build" ]; then
    mkdir app/build
fi

# Build the Elm files
cd frontend
#elm-make Client.elm --output ../app/build/elm-client.js
elm-make Visitor.elm --output ../app/build/elm-visitor.js
cd ..
```
