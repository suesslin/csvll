# CSV Language Manager
Manage multiple languages through CSV tables.

## Getting Started
In this walkthrough, we'll be using **Google Sheets** as a tool.
###Step 1 - Creating a Table
<img src="https://lh6.googleusercontent.com/Qf1VnlLKBEDnvlRkk5dbP1zxyczKeuXINvoyHj-EsGvJkUBUI4DmL_xzO5_hmd0kAUR9Ux3BV3K-DvA=w2880-h1606-rw" width=50%>

As you can see, a table with IDs in the first column, and languages in the first row should be created. This should be
relatively easy to comprehend.

###Step 2 - Save Table as **.csv**
<img src="https://lh3.googleusercontent.com/_epaI9HY_4oGvadPLJRCgmWdQRevSPWO66z6iYQOeK97EIMqpE7wYTlUmUb-iemE80vZd9w0Kaor89Y=w2880-h1606-rw" width=50%>

###Step 3 - Add **csvlm** as Dependency
1. In your *cargo.toml* add
```Rust
[dependencies]
// Assign latest version (Might not be the one saying)
csvlm = "0.1.2"
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
