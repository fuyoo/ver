# Ver

> A cargo subcommand toolkit to manage cargo project version

## Features

+ **prerelease**

  > 0.0.1 -> 0.0.1-0 <br/>
  > 0.0.1-0 -> 0.0.1-1
 
+ **prepatch**

  > 0.0.1 -> 0.0.2-0 <br/>
  > 0.0.1-0 -> 0.0.2-0

+ **preminor**

  > 0.0.1 -> 0.1.1-0 <br/>
  > 0.1.1-0 -> 0.2.1-0

+ **premajor**
  > 0.0.1 -> 1.0.1-0 <br/>
  > 0.0.1-0 -> 1.0.1-0
  
+ **patch**

  > 0.0.1 -> 0.0.2 <br/>
  > 0.0.1-0 -> 0.0.2
  
+ **minor**

  > 0.0.1 -> 0.1.1 <br/>
  > 0.0.1-0 -> 0.1.1

+ **major**

  > 0.0.1 -> 1.0.1 <br/>
  > 0.0.1-0 -> 1.0.1


## Usage

1. cargo install cargo-dover
2. at your Cargo.toml dir execute `cargo ver [prerelease,patch,...]`



