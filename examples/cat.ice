// Repeatedly echos the contents of STDIN to STDOUT

loop {
    let message = input();

    if(message == null) {
        break;
    }
    else {
        println(message);
    };
};