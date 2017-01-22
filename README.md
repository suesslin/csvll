# CSV Language Manager
Manage multiple languages through CSV tables.

**Links**:
- [GitHub](https://github.com/paziphik/csvlm)
- [Cargo](https://crates.io/crates/csvlm)

## Getting Started
In this walkthrough, we'll be using **Google Sheets** as a tool.
###Step 1 - Creating a Table
<img src="http://luke.guru/2jF1Aqq" width=50%>

As you can see, a table with IDs in the first column, and languages in the first row should be created. This should be
relatively easy to comprehend.

###Step 2 - Save Table as **.csv**
<img src="http://luke.guru/2iQO5XK" width=50%>

###Step 3 - Add **csvlm** as Dependency
1. In your *cargo.toml* add
```Rust
[dependencies]
// Assign latest version (Might not be the one saying)
csvlm = "0.1.3"
```
<br>
2. In the command line run
`cargo install`
<br>
<br>
3. In your executable/library of choice add
```Rust
extern crate csvlm;

use csvlm::Manager;
```

###Step 4 - Create Manager & Parse
Now we need a manager that parses the information for us
```Rust
// The parameters are directory, filename & extension
// My file is located outside of the project
let mut manag = Manager::new("..", "test_table", ".txt");

// Then parse the file assigned
manag.parse();
```
