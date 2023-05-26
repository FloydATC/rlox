



1. Overview

    RLOX is an experimental imperative dynamically typed lexically scoped scripting language 
    originally based upon Robert Nystrom's LOX language from https://craftinginterpreters.com 
    which supports functions, closures, classes and instances.

    This implementation written in Rust is radically different in many ways but 
    I must emphasize: It would never have been possible without the lessons from that book.

    Moreover, I am learning Rust as I'm playing around with this so
    I apologize in advance from any mental injury caused by the code herein.


2. Syntax

    2.1. Comments

        Two types of comments are supported: Line comments and block comments.
        Both types are completely ignored by the parser.

        2.1.1. Line comments

            A line comment begins with '//' and ends at the end of that line.

            var name = "Bob"; // This text is ignored.


        2.1.2. 

            A block commend begins with '/*', ends with the first '*/' encountered 
            and may span multiple lines. Note that it is not possible to nest one
            block comment within another.

            /* 
                Roses are read, violets are blue
                Comments are where, false can be true
                                                       */


    2.2. Statements

        As a general rule, every statements must end with a semicolon ';' 
        except when followed by a right curly brace '}',
        ignoring whitespace and comments.

        2.2.1. Variable declarations

            Named variables must be declared using the keyword 'var' before use.
            The variable type is automatically inferred and may change dynamically.
            Any expression may be assigned to a variable, including classes, instances,
            functions, methods and arrays.

            var number = 123.4;
            number = 234.5;

            var name = "Bob";
            name = "Alice";

            var something = number;
            something = name;


            The following variable types are supported:

            - 'null'
            - Boolean 'true' and 'false'
            - Numbers that can be represented by a 64-bit IEEE floating-point value

            - Strings containing 0 or more valid UTF8 code points
            - Functions, Classes and Instances
            - Arrays containing 0 or more values of any supported types. Arrays are non-homogenous, 
            they may be nested and may even be recursive.

            The first three types are always passed by value.
            The rest are always passed by reference.

            // The following code prints "foobar" because a and b reference the same string
            var a="foo"; var b=a; a.push("bar"); print b; 

            // The following code prints "foo" because a and b reference separate strings
            var a="foo"; var b="foo"; a.push("bar"); print b;


        2.2.2. Constant declarations

            Named constants are special variables that can not be changed after they have been declared.

            const taxes = true;
            taxes = false; // <-- Compile error


            Note: An object referenced by a constant can still be changed;
            the reference is constant but the referenced object is not:
            const a="foo"; var b=a; b.push("bar"); print a; // Prints "foobar"!

            In fact, because methods don't care about where their receiver comes from, this works:
            const a="foo"; a.push("bar"); print a; // NOT an error, prints "foobar"!

            This is because methods themselves are first class objects that may be passed around:
            const a="foo"; var m = a.push; m("bar"); print a; // Again, "foobar".


            In practice, constants are probably most useful for numbers.
            const PI=3.14159265;


        2.2.3. Function declaration

            A named function is a re-usable unit of compiled code that can be invoked by calling.

            // Declare a function named 'double' that takes one argument. Within that function,
            // that number is assigned to a variable named 'x'. 
            fun double(x) {
                return x + x;
            }

            print double(2); // Prints the number 4
            print double("foo"); // Prints the string "foofoo"
            print double(null); // Runtime error: Can not add operands null and null

            Functions are first class objects, meaning they can be assigned to variables and passed around.

            fun invoke_what(fx, number) {
                return fx(number);
            }

            invoke_what(double, 2); // Also prints the number 4
            invoke_what("Bob", double); // Runtime error: Can not call a string as a function


        2.2.4. Class declarations

            todo


        2.2.5. Method declarations

            todo


    2.3. Expressions

        todo
        

