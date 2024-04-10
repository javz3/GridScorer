# GridScorer

GridScorer takes in a linear array and turns into an n matrix; based on the input it will calculate the highest score from itself and the surrounding location. This will then return a string result with the following format (x, y, total value)

Example array: `[4,2,3,2,0,1,2,2,1,3,0,2,2,0,1,5]`

Example matrix from the above array:

```
4 | 2 | 3 | 2

0 | 1 | 2 | 2

1 | 3 | 0 | 2

2 | 0 | 1 | 5
```

Example of input:

```
count_of_high_scores = 2

row_length = 4

array = [4,2,3,2,0,1,2,2,1,3,0,2,2,0,1,5]
```

Example of output:

`(1, 2, 17)(1, 1, 16)`

## Setup

Make sure to have rustup installed - https://www.rust-lang.org/tools/install

You can use your IDE of your choice, but I will provide the setup for Visual Studio Code:

1) Install the "rust-analyzer" extension (this aids with development and the ability to build the project)
    - Using the shortcut ctrl + shift + B, you will get the option to build, check or run clippy (conforming to the standard style of Rust) against the project
2) Install "CodeLLDB" this will help running tests locally