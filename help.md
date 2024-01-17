
## USAGE

If you are using the CLI executable directly:
    factorial_calculator option1=value1 option2=value2 option3=value3 ...

If you are using `cargo`:
    cargo run -- option1=value1 option2=value2 option3=value3 ...

## AVAIBLE OPTIONS

### target (required)
Specifies up to what factorial must be calculated.
Must be an integer in the range `[0, 2^64 - 1]`

### save-step (optional)
Specifies how often saves must be made (see the examples at the bottom if you are confused)
Must be an integer in the range `[1, 2^64 - 1]`, if absent only the target is saved

IMPORTANT NOTE:
If there are already other calculated factorials below the target, there will be no saves before the closest saved factorial.
For example, if 2500000 is already calculated locally and you run the calculator with ```target=5000000``` and ```save-step=1000000```,
only 3000000, 4000000 and 5000000 will be saved.

### use-remote-files
Specifies wether the github repository should be used to get factorial files instead of the local repository
If on, it will also only save to the repository.
Must be `yes`/`true` or `no`/`false`, and by default it is disabled

## EXAMPLES

The following will create local files for 1000, 2000, 3000, 4000 and 5000 if there are no other precalculated factorials below 5000:
    factorial_calculator target=5000 save-step=1000

The following will create remote files for 1000000, 2000000 and 3000000 if there are no ther precalculated factorials below 3000000:
    factorial_calculator target=3000000 save-step=1000000 use-remote-files=true