# This is just an idea dump for a blog post
# I am attempting to make a safe way of streaming C data. Where it all comes from a lot of raw points to both the data and configuartions of reading the data

# I have some enums which make it easy and I can make some of these pointer in place which would cut down on some of the pointers. 

# Still some of these pointers need to be kept alive because for example one points to the realsense. Can I wrap that in a reference and then make it an pointer and the function in an unsafe code block. Edit this didn't work. I added unsafe sync thing which made all of my null pointers very visibile

# I do not handle any null pointers and certain am not dropping all my pointers yet. Close but I am guessing I missed more

# Narrowed down some potential sources of error. Looks like there is a corrupt frame? Then when I try to extract all the information from that frame I get a lot of errors, which I am guessing lead to a null pointer for the data. I try to make a slice and call .get_unchecked which should just explode and seemingly does. 

# Need to rewrite everything with options? so I can return them on errors. I do not know how often this should happen. Like if I try to poll the realsense faster than 30fps will I just get bad frames? Why is the pointer not null but I cannot extract info from it

# This is a little cursed. I will now try and read the docs, source code and code for the visualizer application to see if they handle any sort frame dropping?