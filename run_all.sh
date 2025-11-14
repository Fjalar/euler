cargo build --release

time for sol in $(find ./target/release -maxdepth 1 -type f -executable)
do
    echo "
--------------------
$sol:
    "
    time $sol
done
