# Compute the x'th fibonacci number.
fn fib(x)
{
    if x < 3
    {
        return 1
    }
    else
    {
        return fib(x-1)+fib(x-2)
    }
}

# This expression will compute the 40th number.
fib(40)
