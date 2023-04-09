# This is just an idea dump for a blog post
# I am attempting to make a safe way of streaming C data. Where it all comes from a lot of raw points to both the data and configuartions of reading the data

# I have some enums which make it easy and I can make some of these pointer in place which would cut down on some of the pointers. 

# Still some of these pointers need to be kept alive because for example one points to the realsense. Can I wrap that in a reference and then make it an pointer and the function in an unsafe code block