# cargo-clean
Instead of Running cargo clean in each directories which you clone other projects testing. 
this is what I just wrote on purpose to make my life eaiser when there are multple rust projects.
# Process
    [] 1 we get all subdirectories inside of the one the pass in the command line arge && get init Duration: `Instant::now()`
    [] 2 we checked if the args pass in is valid or not by bounced check with args len() function
    [] 3 we iterate over each directores by spawning a new thread and running cargo clean in that thread.
    [] 4 after we all the those finished. we joined all the thread by calling `join()` on each JoinHandler return from the each single threads.
    [] 5 we get the elapes() which return the Duration && run du -sh . script again to get the current size of directory we want to clean up .
    