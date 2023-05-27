



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

            Object classes are declared using the keyword 'class', followed by the class name,
            then optionally the keyword 'of' and a superclass to inherit methods from, then 
            finally in curly braces each method is declared.

            class Human {
                // Methods go here
            }

            class Female of Human {
                // Methods go here, overloading any superclass methods with the same name
            }


        2.2.5. Method declarations

            Methods are declared on an object class but called on an object instance:

            class Animal {
                speak() { 
                    print this.sound(); // The keyword 'this' refers to the object instance itself
                }
            }

            class Dog of Animal {
                sound() {
                    return "woof!";
                }
            }

            class Cow of Animal {
                sound() {
                    return "moo!";
                }
            }

            Calling a class creates an object instance. Methods can be then be called:

            var fido = Dog(); // 'fido' is now an instance of the class Dog, a subclass of Animal.
            fido.speak(); // Prints "woof!" because Dog inherited the 'speak' method.


            Note: In the above example, it is possible to create an instance of Animal, 
            but calling 'speak' method on such an instance would cause a runtime error:

            var aardvark = Animal();
            aardvark.speak(); // Error: Animal does not have a method or field 'speak'!


        2.2.5.1 Method arguments

            Like functions, methods may take arguments. If the constructor method 'init' takes 
            arguments, those arguments are required when calling the class to create an instance.


        2.2.5.2 Special variables 'this' and 'super'

            As mentioned above, object methods may refer to the instance itself as 'this',
            a special variable that is only valid inside a method declaration.

            Object instances can have fields, classes can not. Methods can be overloaded and superclass methods
            may be accessed via 'super'. Notice the subtle difference between 'this' and 'super';
            - 'this' refers to an object instance (that may have both methods and fields)
            - 'super' refers to a class (that can only have methods)

            class Cat of Animal {
                sound() {
                    return "miau!";
                }

                // A method named 'init' gets called automatically on new instances as a constructor
                init() {
                    this.times_spoken = 0; // Creates an instance field named 'times_spoken'
                    return; // A constructor can not return a value
                }

                speak() {
                    var local = 123; // Local variables (and constants) are temporary
                    this.times_spoken = this.times_spoken + 1;
                    super.speak(); // Call the method 'speak' as defined on the superclass
                }
            }


    2.3. Statements

        2.3.1. Print statement

            You have already seen the 'print' statement, it works the same way as in most
            other programming languages; the expression that follows gets printed.


        2.3.2. Return statement

            Functions and methods can produce one or both of the following: 
            A side effect and/or a return value.

            Examples of side effects may be printing something, sending/receiving something
            over the network, altering the program state by changing a variable etc.

            A return value can be thought of as output from a function, for example 
            a function named 'rand' might produce a random number each time it is called, and
            a function named 'sin' would typically take one argument (an angle) and produce
            the trigonometric sinus of that angle.

            A simpler example is a function that produces the double of its input argument:

            fun double(x) {
                return x * 2; // Return whatever number x it is called with, multiplied by two.
            }

            print double(5); // Prints 10, the value returned by the 'double' function as called.


        2.3.3. Exit statement

            The exit statement immediately halts the script. Optionally. you can exit with an
            integer "result code" that can sometimes be interpreted by the operating system
            depending on the context.
            
            exit; // Implies result code 0, which in Windows and Linux means "No error".
            exit 2; // In Windows and Linux, result code 2 usually means "No such file or directory".


        2.3.4. If statement

            Referred to as "branching", 'if' statements let a script alter its behaviour based upon
            whether a conditional expression is 'true' or 'false'. In the following example, 
            we want to check if the variable 'name' contains the string "Bob" or not:

            if (name == "Joshua") {
                print "Shall we play a game?"; // Prints only if the condition is 'true'
            }

            The conditional expression must be enclosed in parenthesis.


            2.3.4.1. If..Else statement

                A slight variation of the If statement is the If..Else statement, which lets you add
                code that gets executed only if the conditional expression is 'false':

                if (name == "Joshua") {
                    print "Shall we play a game?"; // Prints only if the condition is 'true'
                } else {
                    print "Strange game"; // Prints only if the condition is 'false'
                }


            2.3.4.2. Negated conditionals: 'if not'

                It is possible to use the keyword 'not' immediately after 'if' to negate the 
                conditional expression. This can sometimes make code slightly more readable.
                Consider these three lines of code; they all to the exact same thing:

                if not (name == "Joshua") print "Strange game";

                if (name != "Joshua") print "Strange game"; // != means 'not equal'

                if (!(name == "Joshua")) print "Strange game"; // ! means boolean 'not'


                Beware though, this should be used with great care. Combining 'if not' with 'else' 
                and/or other means of negation is technically possible but it's a quick and easy
                way to produce code that's hard for humans to understand:

                if not (false == !(name != "Bob")) print "Yes, please stop";
                else print "No, you're free to go";


            2.3.4.3. A note on assignment vs. comparison

                It's important to notice that '=' means 'assign', as in copy a value from right to left,
                while '==' means 'is equal' as in comparison. Observe the difference because this is 
                a common cause of bugs. In fact, many programming languages will warn you about this
                but RLOX will not! 
                
                Here, the string "Bob" is assigned to 'name', replacing whatever 'name' contained before:

                if (name = "Joshua") {
                    // Because "Bob" is considered a 'true' value, this will always get printed
                    print "Shall we play a game?"; 
                } else {
                    // This can never get printed
                    print "Strange game"; 
                }


            2.3.4.4. Code blocks vs. single statements

                In the above examples, the so-called 'then' blocks (which get executed only if the 
                conditional expression is 'true') and the so-called 'else' blocks (which get executed 
                if the conditional expression is 'false') have all been enclosed in curly braces.

                If a block only contains a single expression, this is not strictly necessary.
                The following code is valid and equivalent:

                if (name = "Joshua") print "Shall we play a game?";
                else print "Strange game";


                Always using braces out of habit makes it easy to add more statements later on. 
                On the other hand, not using braces can sometimes make the code easier to read.


        2.3.5. Loops

            Loops are used to perform and action or set of actions more than one time.
            RLOX supports three different types of loops but it's important to keep in mind that
            they are all just variations of the same idea.


        2.3.5.1. While loops

            Similar to an 'if'-statement, a 'while'-loop differs in that the 'then' block is 
            executed again and again for as long as the conditional expression remains true.

            while (name == "Joshua") {
                print "Shall we play a game?";
            }

            Because there is nothing to change the contents of 'name', the loop would
            keep printing the same line over and over until the script is stopped, but let's say we
            had some function that returns names from some outside source?

            var name;
            while (name = get_next()) {
                print "Shall we play a game?";
            }

            Now, for as long as the function returns a name, the loop will keep running but as soon as
            our imaginary function 'get_next' returns something considered to be 'false', such as an 
            empty string, the number 0, a boolean 'false' or 'null', the loop ends.


            In the previous example, a variable named 'name' was declared immediately before
            the 'while' loop. This may sometimes be what you want, perhaps you need that variable
            in the code that follows after the 'while'-loop? Other times, you may want to use
            a variable that only exists within the loop -- we call this a "locally scoped" variable:

            while (var name = get_next()) {
                print "Shall we play a game?";
            }

            Here, that's exactly what happens. The variable 'name' will now only exist within the loop.            


            As with 'if'-statements, parenthesis are required around the conditional expression, 
            and the curly braces are not strictly needed if there is only one statement:

            while (name = get_next()) print "Shall we play a game?";

            Unlike 'if'-statements however, 'while'-statements do not accept an 'else' block.


            // 'true' will always be 'true' so this is an infinite loop
            while (true) print "Waiting for the universe to go cold";


        2.3.5.2. For loops (C-style) 

            Be warned, the following is pretty dense so unless you're already familiar with C-style loops,
            you may want to re-read this section a few times and perhaps come back to it later for reference.
            Once you've cracked them, C-style loops are incredibly powerful and not really that different
            from actual code using 'while'-loops.

            Ahem. The exact grammar is:

            for (INITIALIZER; CONDITIONAL; INCREMENT) STATEMENT;

            Let's break it down:

            Step 1: The INITIALIZER is a statement or block that, if present, runs exactly one time, before entering the loop.
            Step 2: The CONDITIONAL is an expression which, if present, will terminate the loop if 'false'.
            Step 3: The STATEMENT will then run once if the CONDITIONAL was 'true'.
            Step 4: The INCREMENT statement or block, if present, will then run once.
            Step 5: Loop back to step 2.

            There are quite a few nuances here, so let's look at a minimal loop to see how it behaves (and why):

            for (;;) print "Waiting for the universe to go cold";

            Step 1: There is no INITIALIZER.
            Step 2: There is no CONDITIONAL, so the loop does not terminate.
            Step 3: Print "Waiting for the universe to go cold"
            Step 4: There is no INCREMENT.
            Step 5: Loop back to step 2.

            See? That was just a slightly more complicated infinite loop! (But we saved 4 characters, yay!)

            Let's do an example you're more likely to encounter in the real world:

            for (var i=0; i<5; i=i+1) print i;

            Step 1: var i=0; // Declare a variable 'i' and assign the number 0, initializing it.
            Step 2: Compare 'i' to see if the number is smaller than 5. If not, terminate the loop.
            Step 3: Print the number in the variable 'i'.
            Step 4: i=i+1; // Increment the number in the variable 'i'.
            Step 5: Loop back to step 2.

            The result? The code would print the numbers 0, 1, 2, 3, 4 before terminating the loop.


            When you break it down, they're not really that hard but they do come with the risk of
            not just infinite loops but also an entire class of software bugs commonly referred to 
            as "off-by-one errors". 
            
            Which is why modern programming languages often come with a third type of loops.


        2.3.5.3. For..In loops (Iterator style)

            Iterator style loops are different from 'while'-loops and C-style loops in that they
            are controlled by some type of input data that they are said to "iterate over".

            RLOX can iterate over three different types of data: Arrays, lists and object instances.

            var array = [123, 456, 789]; // Variable 'array' with three numbers in it
            for var number in array {
                print number;
            }
            
            Before entering the loop, a variable named 'number' is declared. Just like in the 
            'while'-loop example, this variable will only exist for as long as the loop executes.
            The loop then takes each of the numbers in the array, assigns it to 'number' and
            runs the print statement. The loop "knows" when to terminate; there's no risk of
            accidentally looping forever or terminating the loop before all numbers have been printed.

            We could write the same code using a list:

            for var number in (123, 456, 789) {
                print number;
            }

            The only difference here is that the numbers are listed directly in the code as
            opposed to an array variable (which could have gotten the numbers from anywhere).


            Finally, iterating over an object instance requires that the object has a method named 'next',
            which takes exactly one argument: The previous value that was returned by next(), or 'null'.
            The loop terminates when next() returns 'null'.

            let foo = Bar(); // Bar must implement 'next' or this won't work.
            for var item in foo {
                print item;
            }

            Notice that we don't actually mention next() anywhere. That's because all the 
            stuff related to passing the previous value and checking for 'null' happens 
            behind the scenes; all you have to worry about is make sure Bar implements
            that method and each value will get assigned to the variable named 'item'
            and then the loop body executes.


    2.4. Expressions

        todo
        

