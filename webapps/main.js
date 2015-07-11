"use strict";

class User {
    constructor(name) {
        this.name = name;
    }
    
    greet() {
        return "Hi, my name is " + this.name;
    }
}

const bob = new User("Bob");
window.alert(bob.greet());