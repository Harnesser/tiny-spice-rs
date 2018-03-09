Writing a SPICE simulator -- A Tutorial
=======================================

Why am I writing this tutorial?
---------------------------------------
* By teaching, I learn in-depth
* Portfolio piece if I ever get made redundant and job searching
* I haven't seen such a tutorial, so it fills a gap
* I enjoy following along at home with PhilOS and the like, so give back
* Might use it as a basis for an internal talk series


Target Audience
---------------------------------------

1. Electronics enthusiast who wonder about the internal wonder about the
  internal workings of the simulators they use to verify their designs,
  and like me, are a bit scared of `RELTOL` and time-step too small etc.

2. Programmers who like follow-along type blog series


What this is not
---------------------------------------
1. A Rust tutorial
2. A tutorial on how to use SPICE
3. An electronics tutorial



Programming Language Choice
---------------------------------------
The programming languages I'd be most comfortable targeting this tutorial to 
are:

#### Python
Pros:
* lots more people would be familiar with it
* lots more people are likely to have it installed

Cons:
* (none for this application?)

Speed isn't an issue here as it's just a toy simulator we're writing.


#### C
Pros:
* lots of people would be familiar with it 
* For EEs who have formal training, they've very likely been exposed to CA

Cons:
* makefiles
* crashes and memory management
* poor module story compared to nearly everything else


#### Go
I've written a tiny verilog engine in Go, and did the exercises to the Coursera
logic to layout course in this language. 

Pros:
* Nicer C
* Blazingly fast compile times

Cons:
* maybe not a huge familiarity quotient


#### Rust

Pros:
* My favourite

Cons:
* Not many people familiar with it
* New tool to install
* Longish compile times
* Steep learning curve


Release Early, Release Often
----------------------------------------
Maybe I should advertise this readme and outline online and see if I get
any bites?

Let's get a chapter or two done first, and take it from there...



References
----------------------------------------

 [1] www.rust.org
 [2] www.python.org

