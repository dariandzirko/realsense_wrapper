# This is just an idea dump for a blog post
# I am attempting to make a safe way of streaming C data. Where it all comes from a lot of raw points to both the data and configuartions of reading the data

# I have some enums which make it easy and I can make some of these pointer in place which would cut down on some of the pointers. 

# Still some of these pointers need to be kept alive because for example one points to the realsense. Can I wrap that in a reference and then make it an pointer and the function in an unsafe code block. Edit this didn't work. I added unsafe sync thing which made all of my null pointers very visibile

# I do not handle any null pointers and certain am not dropping all my pointers yet. Close but I am guessing I missed more

# Narrowed down some potential sources of error. Looks like there is a corrupt frame? Then when I try to extract all the information from that frame I get a lot of errors, which I am guessing lead to a null pointer for the data. I try to make a slice and call .get_unchecked which should just explode and seemingly does. 

# Need to rewrite everything with options? so I can return them on errors. I do not know how often this should happen. Like if I try to poll the realsense faster than 30fps will I just get bad frames? Why is the pointer not null but I cannot extract info from it

# This is a little cursed. I will now try and read the docs, source code and code for the visualizer application to see if they handle any sort frame dropping?

# Options was not what I was looking for lol. I wanted Results so I am rewriting this to take results and handle the Results fro checking the errors

# Results are popping off, but we started mid-way down the pipeline leading to issues. Now time to start at the top of the pipeline and get the instance of literal realsense as a nice Result created thing. Time to move all of the example code out of the unsafe block. Wow this is a lot of mental visualization and energy to get this to a point that I would want.

# Okay so the current commit-2 is working pretty well, and it did not does crash but it just updates super slowly. Current idea is now make a big queue and hopefully update side by side so it's not super duper slow

# Now I potentially fixed the framerate and resolution but the system in bevy will only run/work when I consistently give it inputs to the system, causing the update system to only trigger on events ie spacebar, mouse movement or clicks