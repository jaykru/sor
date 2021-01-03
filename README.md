# sor
a companion to https://github.com/google/walk

shell-or: takes a stream of new-line separated filenames from standard input. for each filename, short-circuit-or evaluates bash snippet tests against those filenames, printing the filename exactly when any of the tests succeeds.

## installing
```
cd $HOME
cargo install --git https://github.com/jaykru/sor
```

## usage
```
walk | sor 'test -f'
```
